export enum WsClientMessageType {
  Auth = 'Auth',
}

export interface IWsClientMessage {
  type: WsClientMessageType
  value?: unknown
}
