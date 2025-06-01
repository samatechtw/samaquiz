import { IQuizSessionViewModel } from './i-quiz-session.view-model'

export interface IListQuizSessionsApiResponse {
  total: number
  results: IQuizSessionViewModel[]
}
