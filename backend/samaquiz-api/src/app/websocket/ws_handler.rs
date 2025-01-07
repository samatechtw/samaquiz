use std::ops::ControlFlow;

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Path, State,
    },
    response::IntoResponse,
};

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::sync::mpsc::{self, Sender};
use uuid::Uuid;

use crate::{
    api_context::ApiContext,
    app::websocket::{
        ws_helpers::is_session_owner,
        ws_state::{get_broadcaster, WsClientMessage, WsReceiverType, WsServerMessage},
    },
};

async fn send_server_message(sender: &Sender<String>, msg: &WsServerMessage) -> Result<(), ()> {
    let Ok(msg_str) = serde_json::to_string(&msg) else {
        println!("Failed to send broadcast");
        return Err(());
    };
    if sender.send(msg_str).await.is_err() {
        // break on err
        println!("send_server_message error");
        return Err(());
    }
    Ok(())
}

pub async fn ws_handler(
    Path(session_id): Path<Uuid>,
    State(context): State<ApiContext>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, context, session_id))
}

async fn handle_socket(mut socket: WebSocket, context: ApiContext, session_id: Uuid) {
    let (mut sender, mut receiver) = socket.split();
    let (sender_mpsc, mut receiver_mpsc) = mpsc::channel::<String>(16);

    tokio::spawn(async move {
        while let Some(message) = receiver_mpsc.recv().await {
            if sender.send(message.into()).await.is_err() {
                break;
            }
        }
    });

    let context_auth = context.clone();
    let session_id_auth = session_id.clone();

    let mut is_host = false;

    // Let the client know whether it's a participant or host based on Auth message
    let mut ready_msg = WsServerMessage::ready_participant();
    // Loop until an auth message is found.
    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(msg_raw) = message {
            let Ok(msg) = serde_json::from_str::<WsClientMessage>(msg_raw.as_str().into()) else {
                println!("First message not WsClientMessage: {}", msg_raw);
                break;
            };
            match msg {
                WsClientMessage::Auth(auth) => {
                    is_host = is_session_owner(&context_auth, session_id_auth, auth.value).await;
                    if is_host {
                        ready_msg = WsServerMessage::ready_host();
                    }
                    break;
                } /*
                  m => {
                      println!("First message was not auth: {:?}", m);
                      break;
                  }
                  */
            }
        };
    }
    // Send ready message to current client
    let _ = send_server_message(&sender_mpsc, &ready_msg).await;

    let receiver_name = "unknown";

    let broadcaster = match get_broadcaster(&context.ws_state, session_id.to_string()) {
        Ok(b) => b,
        Err(e) => {
            println!("Failed to get broadcaster: {}", e);
            return;
        }
    };
    let mut broadcast_receiver = broadcaster.subscribe();

    let sender_clone = sender_mpsc.clone();

    let mut send_task = tokio::spawn(async move {
        let mut send_count = 0;
        while let Ok(msg) = broadcast_receiver.recv().await {
            send_count += 1;
            match msg {
                WsServerMessage::Joined(_) => {
                    if is_host {
                        if send_server_message(&sender_clone, &msg).await.is_err() {
                            break;
                        }
                    }
                }
                WsServerMessage::QuizResponse(_) => {
                    if is_host {
                        if send_server_message(&sender_clone, &msg).await.is_err() {
                            break;
                        }
                    }
                }
                m => {
                    if send_server_message(&sender_clone, &m).await.is_err() {
                        break;
                    }
                }
            }
        }
        send_count
    });

    let mut recv_task = tokio::spawn(async move {
        let mut send_count = 0;
        while let Some(Ok(msg)) = receiver.next().await {
            send_count += 1;
            // print message and break if instructed to do so
            if process_message(msg).is_break() {
                break;
            }
        }
        send_count
    });

    // If either task exits, abort the other.
    tokio::select! {
        rv_a = (&mut send_task) => {
            match rv_a {
                Ok(a) => println!("{a} messages sent"),
                Err(a) => println!("Error sending messages {a:?}")
            }
            recv_task.abort();
        },
        rv_b = (&mut recv_task) => {
            match rv_b {
                Ok(b) => println!("Received {b} messages"),
                Err(b) => println!("Error receiving messages {b:?}")
            }
            send_task.abort();
        }
    }

    // returning from the handler closes the websocket connection
    println!("Websocket context {receiver_name} destroyed");
}

fn process_message(msg: Message) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> got str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> got {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> got close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> somehow got close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> got pong with {v:?}");
        }
        _ => {}
    }
    // TODO --client messages
    /*
    let Ok(msg) = serde_json::from_str::<WsHostMessage>(msg.as_str()) else {
        return ControlFlow::Continue(());
    };
    match msg {
        Message::Text(t) => {
            println!(">>> got str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> got {} bytes: {:?}", d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> got close with code {} and reason `{}`",
                    cf.code, cf.reason
                );
            } else {
                println!(">>> somehow got close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> got pong with {v:?}");
        }
        _ => {}
    }
     */
    ControlFlow::Continue(())
}
