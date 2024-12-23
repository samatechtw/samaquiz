import { IQuizViewModel } from './i-quiz.view-model'

export interface IListQuizzesApiResponse {
  total: number
  results: IQuizViewModel[]
}
