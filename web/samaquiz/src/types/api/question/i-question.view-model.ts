import { AssetContentType } from '../asset'
import { QuestionType } from './enum-question-type'

export interface IQuestionAssetViewModelRelation {
  id: string
  content_type: AssetContentType
  size: number
  project_id: string
}

export interface IQuestionViewModel {
  id: string
  quiz_id: string
  text: string
  question_type: QuestionType
  answers_order: string[]
  asset?: IQuestionAssetViewModelRelation
  created_at: Date
  updated_at: Date
}
