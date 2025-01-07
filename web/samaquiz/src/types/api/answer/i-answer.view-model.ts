export interface IAnswerViewModel {
  id: string
  question_id: string
  text: string
  is_correct: boolean
  points: number
  created_at: Date
  updated_at: Date
}
