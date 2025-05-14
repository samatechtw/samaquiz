import { QuizType } from './enum-quiz-type'

export interface IQuizViewModel {
  id: string
  user_id: string
  title: string
  description: string
  quiz_type: QuizType
  questions_order: string[]
  created_at: Date
  updated_at: Date
}
