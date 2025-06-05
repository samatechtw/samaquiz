import {
  ICreateQuizAssetRequest,
  ICreateQuizAssetResponse,
  IListQuizAssetsRequest,
  IListQuizAssetsResponse,
  IUpdateQuizAssetRequest,
  IUpdateQuizAssetResponse,
  IVerifyQuizAssetResponse,
} from '@frontend/types'
import { RequestParams } from '@samatech/fetch-api'
import { rootApi } from './root-api'

export const apiCreateQuizAsset = async (
  payload: ICreateQuizAssetRequest,
): Promise<ICreateQuizAssetResponse> => {
  const { data } = await rootApi.authRequest({
    url: 'quiz_assets',
    method: 'POST',
    data: payload,
  })
  return data as ICreateQuizAssetResponse
}

export const apiVerifyQuizAsset = async (
  id: string,
): Promise<IVerifyQuizAssetResponse> => {
  const { data } = await rootApi.authRequest({
    url: `quiz_assets/${id}/actions/verify`,
    method: 'POST',
  })
  return data as IVerifyQuizAssetResponse
}

export const apiUpdateQuizAsset = async (
  id: string,
  payload: IUpdateQuizAssetRequest,
): Promise<IUpdateQuizAssetResponse> => {
  const { data } = await rootApi.authRequest({
    url: `quiz_assets/${id}`,
    method: 'PATCH',
    data: payload,
  })
  return data as IUpdateQuizAssetResponse
}

export const apiListQuizAssets = async (
  params: IListQuizAssetsRequest,
): Promise<IListQuizAssetsResponse> => {
  const { data } = await rootApi.authRequest({
    url: 'quiz_assets',
    method: 'GET',
    params: params as RequestParams,
  })
  return data as IListQuizAssetsResponse
}

export const apiDeleteQuizAsset = async (id: string): Promise<void> => {
  await rootApi.authRequest({
    url: `quiz_assets/${id}`,
    method: 'DELETE',
  })
}

export const apiUploadFileWithSignedUrl = (
  url: string,
  file: File,
): Promise<Response> => {
  return fetch(url, { method: 'PUT', body: file, mode: 'cors' })
}
