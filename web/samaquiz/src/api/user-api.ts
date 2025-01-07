import {
  IGetUserApiResponse,
  ILoginUserApiRequest,
  ILoginUserApiResponse,
  IRegisterUserApiRequest,
  IRegisterUserApiResponse,
  IUpdateUserApiRequest,
  IUpdateUserApiResponse,
} from '@frontend/types'
import { rootApi } from './root-api'

export const apiRegisterUser = async (
  payload: IRegisterUserApiRequest,
): Promise<IRegisterUserApiResponse> => {
  const { data } = await rootApi.authRequest({
    url: 'users/registrations',
    method: 'POST',
    data: payload,
  })
  return data as IRegisterUserApiResponse
}

export const apiLoginUser = async (
  payload: ILoginUserApiRequest,
): Promise<ILoginUserApiResponse> => {
  const { data } = await rootApi.authRequest({
    url: 'auth/logins',
    method: 'POST',
    data: payload,
  })
  return data as ILoginUserApiResponse
}

export const apiGetUser = async (id: string): Promise<IGetUserApiResponse> => {
  const { data } = await rootApi.authRequest({
    url: `users/${id}`,
    method: 'GET',
  })
  return data as IGetUserApiResponse
}

export const apiUpdatePassword = async (jwt: string, password: string): Promise<void> => {
  await rootApi.request({
    url: 'auth/logins/passwords',
    method: 'PATCH',
    data: { password },
    headers: {
      Authorization: `Bearer ${jwt}`,
    },
  })
}

export const apiUpdateUser = async (id: string, payload: IUpdateUserApiRequest) => {
  const { data } = await rootApi.authRequest({
    url: `users/${id}`,
    method: 'PATCH',
    data: payload,
  })
  return data as IUpdateUserApiResponse
}

export const apiResetPassword = async (email: string): Promise<void> => {
  await rootApi.request({
    url: 'auth/logins/reset-password',
    method: 'POST',
    data: { email },
  })
}

export const apiConfirmEmail = async (code: string): Promise<void> => {
  await rootApi.request({
    url: 'auth/confirm-email',
    method: 'POST',
    data: { code },
  })
}

export const apiResendConfirmEmail = async (): Promise<void> => {
  await rootApi.authRequest({
    url: 'auth/resend-confirm-email',
    method: 'POST',
  })
}
