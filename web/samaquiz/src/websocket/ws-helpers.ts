import { IWsClientMessage, WsClientMessageType } from '@frontend/types'

export const sendWsAuth = (ws: WebSocket, token?: string | null) => {
  const auth: IWsClientMessage = {
    type: WsClientMessageType.Auth,
  }
  if (token) {
    auth.value = token
  }
  ws.send(JSON.stringify(auth))
}
