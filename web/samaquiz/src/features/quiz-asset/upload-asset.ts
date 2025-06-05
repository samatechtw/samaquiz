import { apiCreateQuizAsset, apiUploadFileWithSignedUrl } from '@frontend/api'
import {
  extFromContentType,
  ICreateQuizAssetRequest,
  ICreateQuizAssetResponse,
  IQuizAssetViewModel,
} from '@frontend/types'
import { S3_QUIZ_ASSETS_URL } from '@frontend/util/config'

export interface IUploadFileResult extends ICreateQuizAssetResponse {
  url: string
}

export const urlFromAsset = (asset: IQuizAssetViewModel): string => {
  let ext = extFromContentType(asset.content_type)
  ext = ext ? `.${ext}` : ''

  return `${S3_QUIZ_ASSETS_URL}/${asset.user_id}/${asset.id}${ext}`
}

const getSignedUrl = async (
  payload: ICreateQuizAssetRequest,
): Promise<ICreateQuizAssetResponse> => {
  const response = await apiCreateQuizAsset(payload)
  return response
}

export const uploadFileAndGetUrl = async (
  payload: ICreateQuizAssetRequest,
  file: File,
): Promise<IUploadFileResult> => {
  const response = await getSignedUrl(payload)
  const url = response.signed_url
  await apiUploadFileWithSignedUrl(url, file)
  return { ...response, url: urlFromAsset(response) }
}
