import { IAnswerViewModel } from '../answer'
import { QuestionType } from './enum-question-type'
import { IQuestionAssetViewModelRelation } from './i-question.view-model'

export interface IGetQuestionApiResponse {
  id: string
  quiz_id: string
  text: String
  question_type: QuestionType
  answers: IAnswerViewModel[]
  answers_order: string[]
  asset?: IQuestionAssetViewModelRelation
  created_at: Date
  updated_at: Date
}
