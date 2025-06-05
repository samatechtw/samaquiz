import { AssetContentType } from './asset-content-type'

export interface ICreateQuizAssetRequest {
  content_size: number
  content_type: AssetContentType
  quiz_id: string
}
