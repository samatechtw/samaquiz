export enum WsServerMessageType {
  Ready = 'Ready',
  Joined = 'Joined',
  QuizCountdown = 'QuizCountdown',
  QuizStart = 'QuizStart',
  QuestionStart = 'QuestionStart',
  QuestionEndUpdate = 'QuestionEndUpdate',
  QuizResponse = 'QuizResponse',
  QuizEnd = 'QuizEnd',
}

export enum WsReceiverType {
  Host = 'Host',
  Participant = 'Participant',
}

export interface WsReadyData {
  receiver_type: WsReceiverType
}

export interface IWsServerMessage {
  type: WsServerMessageType
  value?: unknown
  question_end_time?: number
  question_index?: number
}
