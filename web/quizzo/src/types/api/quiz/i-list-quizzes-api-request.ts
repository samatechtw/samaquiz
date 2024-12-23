import { SortDirection } from '../shared'
import { QuizType } from './enum-quiz-type'

export interface IListQuizzesApiRequest {
  readonly from?: number
  readonly to?: number
  types?: QuizType[]
  user_id?: string
  column?: 'created_at' | 'updated_at'
  direction?: SortDirection
}
