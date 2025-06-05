import { QuizAssetState } from './quiz-asset-state'

export interface IQuizAssetViewModel {
  id: string
  size: number
  content_type: string
  state: QuizAssetState
  user_id: string
  quiz_id: string
  upload_expires_at: Date
  created_at: Date
}
