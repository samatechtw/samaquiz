import { IQuestionViewModel } from '../question'
import { IQuizSessionViewModel } from '../quiz-session'
import { QuizType } from './enum-quiz-type'

export interface IGetQuizApiResponse {
  id: string
  user_id: string
  title: string
  description: string
  quiz_type: QuizType
  questions: IQuestionViewModel[]
  sessions: IQuizSessionViewModel[]
  questions_order: string[]
  intro_background_url: string
  created_at: Date
  updated_at: Date
}
