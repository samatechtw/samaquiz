import {
  ICreateQuizSessionApiRequest,
  ICreateQuizSessionApiResponse,
  IGetParticipantCountApiResponse,
  IGetQuizSessionApiResponse,
  IGetSessionLeadersApiResponse,
  IListQuizSessionsApiRequest,
  IListQuizSessionsApiResponse,
  IUpdateQuizSessionApiRequest,
} from '@frontend/types'
import { rootApi } from './root-api'
import { RequestParams } from '@samatech/fetch-api'

export const apiGetSession = async (
  code: string,
): Promise<IGetQuizSessionApiResponse> => {
  const { data } = await rootApi.authOptRequest<IGetQuizSessionApiResponse>({
    url: `quiz_sessions/code/${code}`,
    method: 'GET',
  })
  return data
}

export const apiListQuizSessions = async (
  query: IListQuizSessionsApiRequest,
): Promise<IListQuizSessionsApiResponse> => {
  const { data } = await rootApi.authRequest<IListQuizSessionsApiResponse>({
    url: 'quiz_sessions',
    method: 'GET',
    params: query as unknown as RequestParams,
  })
  return data
}

export const apiCreateSession = async (
  quizId: string,
  payload: ICreateQuizSessionApiRequest,
): Promise<ICreateQuizSessionApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateQuizSessionApiResponse>({
    url: `quizzes/${quizId}/sessions`,
    method: 'POST',
    data: payload,
  })
  return data
}

export const apiUpdateSession = async (
  id: string,
  payload: IUpdateQuizSessionApiRequest,
): Promise<void> => {
  await rootApi.authRequest({
    url: `quiz_sessions/${id}`,
    method: 'PATCH',
    data: payload,
  })
}

export const apiGetParticipantCount = async (
  sessionId: string,
): Promise<IGetParticipantCountApiResponse> => {
  const { data } = await rootApi.authRequest<IGetParticipantCountApiResponse>({
    url: `quiz_sessions/${sessionId}/queries/participant_count`,
    method: 'GET',
  })
  return data
}

export const apiGetSessionLeaders = async (
  sessionId: string,
): Promise<IGetSessionLeadersApiResponse> => {
  const { data } = await rootApi.authRequest<IGetSessionLeadersApiResponse>({
    url: `quiz_sessions/${sessionId}/queries/leaders`,
    method: 'GET',
  })
  return data
}
