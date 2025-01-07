import { IQuestionViewModel } from '../question'
import { QuizType } from '../quiz/enum-quiz-type'
import { QuizSessionStatus } from './enum-quiz-session-status'

export interface IQuizSessionQuizRelation {
  id: string
  user_id: string
  title: string
  description: string
  quiz_type: QuizType
  questions: IQuestionViewModel[]
  questions_order: string[]
  created_at: Date
  updated_at: Date
}

export interface IQuizSessionParticipantRelation {
  id: string
  name: string
  avatar: string
  created_at: Date
}

export interface IGetQuizSessionApiResponse {
  id: string
  user_id: string
  quiz: IQuizSessionQuizRelation
  host_name: string
  host_avatar: string
  code: string
  start_time?: number
  end_time?: number
  question_end_time?: number
  question_index?: number
  question_duration: number
  status: QuizSessionStatus
  participants?: IQuizSessionParticipantRelation[]
  created_at: Date
  updated_at: Date
}
