import { AssetContentType } from './asset-content-type'

export interface ICreateAssetRequest {
  content_size: number
  content_type: AssetContentType
  project_id: string
}

export interface ICreateRewardAssetRequest {
  content_size: number
  content_type: AssetContentType
  reward_id: string
}
