import { QuizSessionStatus } from './enum-quiz-session-status'

export interface IQuizSessionViewModel {
  id: string
  quiz_id: string
  code: string
  start_time: number
  end_time: number
  status: QuizSessionStatus
  created_at: Date
  updated_at: Date
}
