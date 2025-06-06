import {
  ICreateQuizAssetRequest,
  IListQuizAssetsRequest,
  IQuizAssetViewModel,
  IUpdateQuizAssetRequest,
  IVerifyQuizAssetResponse,
} from '@frontend/types'
import { IFeatureParams } from '../i-feature-params'
import { IUploadFileResult, uploadFileAndGetUrl } from './upload-asset'
import { apiListQuizAssets, apiUpdateQuizAsset, apiVerifyQuizAsset } from '@frontend/api'

export interface IListQuizAssetParams extends IFeatureParams {
  assets: IQuizAssetViewModel[]
  usage: number
}

export const createAsset = async (
  data: ICreateQuizAssetRequest,
  file: File,
): Promise<IUploadFileResult> => {
  const result = await uploadFileAndGetUrl(data, file)
  return result
}

export const verifyAsset = async (id: string): Promise<IVerifyQuizAssetResponse> => {
  const result = await apiVerifyQuizAsset(id)
  return result
}

export const updateAsset = async (
  id: string,
  payload: IUpdateQuizAssetRequest,
  params: IFeatureParams,
): Promise<IQuizAssetViewModel | undefined> => {
  params.loading = true
  try {
    const updated = await apiUpdateQuizAsset(id, payload)
    return { ...updated, version: 1 } as IQuizAssetViewModel
  } catch (e) {
    console.log('Update site asset error', e)
  } finally {
    params.loading = false
  }
}

export const listAssets = async (
  payload: IListQuizAssetsRequest,
  params: IListQuizAssetParams,
): Promise<void> => {
  params.loading = true
  try {
    const response = await apiListQuizAssets(payload)
    params.usage = response.total_usage
    params.assets = response.results.map((asset) => ({ ...asset, version: 1 }))
  } catch (e) {
    console.log('List site assets failed', e)
  }
  params.loading = false
}
