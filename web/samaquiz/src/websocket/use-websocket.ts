import { WS_URL } from '@frontend/api'
import { onMounted, onUnmounted } from 'vue'
import { sendWsAuth } from './ws-helpers'
import { store } from '@frontend/store'
import { IWsServerMessage } from '@frontend/types'

export type ServerMessageHandler = (msg: IWsServerMessage) => void

export const useWebsocket = (sessionId: string, handleMessage: ServerMessageHandler) => {
  let ws: WebSocket | undefined

  const tryConnect = () => {
    if (sessionId) {
      ws = connect()
    }
  }

  const connect = () => {
    const url = `${WS_URL}/${sessionId}`
    const ws = new WebSocket(url)

    ws.onopen = function () {
      sendWsAuth(ws, store.auth.token.value)
    }

    ws.onmessage = function (e) {
      const msg = JSON.parse(e.data)
      handleMessage(msg)
    }

    ws.onclose = function (e) {
      console.log('Socket is closed. Reconnect will be attempted in 3 seconds.', e.reason)
      setTimeout(tryConnect, 3000)
    }

    ws.onerror = function (err) {
      console.error('Socket encountered error: ', err, 'Closing socket')
      ws.close()
    }
    return ws
  }

  onMounted(() => {
    if (!ws) {
      tryConnect()
    }
  })

  onUnmounted(() => {
    if (ws) {
      ws.close()
      ws = undefined
    }
  })
}
