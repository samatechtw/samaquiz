import {
  ICreateAnswerApiRequest,
  ICreateAnswerApiResponse,
  IUpdateAnswerApiRequest,
} from '@frontend/types'
import { rootApi } from './root-api'

export const apiDeleteAnswer = async (id: string): Promise<void> => {
  await rootApi.authRequest({
    url: `answers/${id}`,
    method: 'DELETE',
  })
}

export const apiCreateAnswer = async (
  questionId: string,
  payload: ICreateAnswerApiRequest,
): Promise<ICreateAnswerApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateAnswerApiResponse>({
    url: `questions/${questionId}/answers`,
    method: 'POST',
    data: payload,
  })
  return data
}

export const apiUpdateAnswer = async (
  id: string,
  payload: IUpdateAnswerApiRequest,
): Promise<ICreateAnswerApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateAnswerApiResponse>({
    url: `answers/${id}`,
    method: 'PATCH',
    data: payload,
  })
  return data
}
