import { QuizSessionStatus } from './enum-quiz-session-status'

export interface IUpdateQuizSessionApiRequest {
  code?: string
  host_name?: string
  host_avatar?: string
  start_time?: number
  end_time?: number
  question_end_time?: number
  question_index?: number
  question_duration?: number
  status?: QuizSessionStatus
}
