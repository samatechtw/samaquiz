import { SortDirection } from '../shared'

export interface IListQuizSessionsApiRequest {
  readonly from?: number
  readonly to?: number
  user_id?: string
  column?: 'created_at' | 'updated_at'
  direction?: SortDirection
}
