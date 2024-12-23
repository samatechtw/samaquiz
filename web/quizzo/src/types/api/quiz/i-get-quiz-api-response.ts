import { IQuestionViewModel } from "../question";
import { QuizType } from "./enum-quiz-type";

export interface IGetQuizApiResponse {
  id: string,
  user_id: string,
  title: string,
  description: string,
  quiz_type: QuizType,
  questions: IQuestionViewModel[],
  questions_order: string[],
  created_at: Date,
  updated_at: Date,
}
