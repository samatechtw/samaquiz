import {
  ICreateQuizApiRequest,
  ICreateQuizApiResponse,
  IGetQuizApiResponse,
  IListQuizzesApiRequest,
  IListQuizzesApiResponse,
  IUpdateQuizApiRequest,
} from '@frontend/types'
import { rootApi } from './root-api'
import { RequestParams } from '@samatech/fetch-api'

export const apiListQuizzes = async (
  query: IListQuizzesApiRequest,
): Promise<IListQuizzesApiResponse> => {
  const { data } = await rootApi.authRequest<IListQuizzesApiResponse>({
    url: 'quizzes',
    method: 'GET',
    params: query as unknown as RequestParams,
  })
  return data
}

export const apiGetQuiz = async (id: string): Promise<IGetQuizApiResponse> => {
  const { data } = await rootApi.authRequest<IGetQuizApiResponse>({
    url: `quizzes/${id}`,
    method: 'GET',
  })
  return data
}

export const apiDeleteQuiz = async (id: string): Promise<void> => {
  await rootApi.authRequest({
    url: `quizzes/${id}`,
    method: 'DELETE',
  })
}

export const apiCreateQuiz = async (
  payload: ICreateQuizApiRequest,
): Promise<ICreateQuizApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateQuizApiResponse>({
    url: 'quizzes',
    method: 'POST',
    data: payload,
  })
  return data
}

export const apiUpdateQuiz = async (
  id: string,
  payload: IUpdateQuizApiRequest,
): Promise<ICreateQuizApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateQuizApiResponse>({
    url: `quizzes/${id}`,
    method: 'PATCH',
    data: payload,
  })
  return data
}
