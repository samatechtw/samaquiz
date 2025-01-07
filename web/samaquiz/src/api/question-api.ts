import {
  ICreateQuestionApiRequest,
  ICreateQuestionApiResponse,
  IGetQuestionApiResponse,
  IUpdateQuestionApiRequest,
} from '@frontend/types'
import { rootApi } from './root-api'

export const apiDeleteQuestion = async (id: string): Promise<void> => {
  await rootApi.authRequest({
    url: `questions/${id}`,
    method: 'DELETE',
  })
}

export const apiGetQuestion = async (id: string): Promise<IGetQuestionApiResponse> => {
  const { data } = await rootApi.authRequest<IGetQuestionApiResponse>({
    url: `questions/${id}`,
    method: 'GET',
  })
  return data
}

export const apiCreateQuestion = async (
  quizId: string,
  payload: ICreateQuestionApiRequest,
): Promise<ICreateQuestionApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateQuestionApiResponse>({
    url: `quizzes/${quizId}/questions`,
    method: 'POST',
    data: payload,
  })
  return data
}

export const apiUpdateQuestion = async (
  id: string,
  payload: IUpdateQuestionApiRequest,
): Promise<void> => {
  await rootApi.authRequest({
    url: `questions/${id}`,
    method: 'PATCH',
    data: payload,
  })
}
