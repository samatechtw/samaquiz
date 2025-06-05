import { QuestionType } from './enum-question-type'

export interface IQuestionViewModel {
  id: string
  quiz_id: string
  text: string
  question_type: QuestionType
  answers_order: string[]
  asset_url: string
  created_at: Date
  updated_at: Date
}
