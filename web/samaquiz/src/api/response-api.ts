import {
  ICreateQuizResponseApiRequest,
  ICreateQuizResponseApiResponse,
} from '@frontend/types'
import { rootApi } from './root-api'

export const apiCreateQuizResponse = async (
  payload: ICreateQuizResponseApiRequest,
): Promise<ICreateQuizResponseApiResponse> => {
  const { data } = await rootApi.authRequest<ICreateQuizResponseApiResponse>({
    url: 'quiz_responses',
    method: 'POST',
    data: payload,
  })
  return data
}
