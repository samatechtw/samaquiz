import {
  ICreateParticipantApiRequest,
  ICreateParticipantApiResponse,
  IGetParticipantApiResponse,
} from '@frontend/types'
import { rootApi } from './root-api'

export const apiCreateParticipant = async (
  sessionId: string,
  payload: ICreateParticipantApiRequest,
): Promise<ICreateParticipantApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateParticipantApiResponse>({
    url: `quiz_sessions/${sessionId}/join`,
    method: 'POST',
    data: payload,
  })
  return data
}

export const apiGetParticipant = async (
  participantId: string,
): Promise<IGetParticipantApiResponse> => {
  const { data } = await rootApi.authRequest<IGetParticipantApiResponse>({
    url: `participants/${participantId}`,
    method: 'GET',
  })
  return data
}
