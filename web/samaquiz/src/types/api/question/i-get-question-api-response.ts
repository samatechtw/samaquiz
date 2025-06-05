import { IAnswerViewModel } from '../answer'
import { QuestionType } from './enum-question-type'

export interface IGetQuestionApiResponse {
  id: string
  quiz_id: string
  text: string
  question_type: QuestionType
  answers: IAnswerViewModel[]
  answers_order: string[]
  asset_url: string
  created_at: Date
  updated_at: Date
}
