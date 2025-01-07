import { AssetContentType } from './asset-content-type'
import { AssetState } from './asset-state'

export interface IAssetViewModel {
  id: string
  size: number
  content_type: AssetContentType
  state: AssetState
  user_id: string
  project_id: string
  upload_expires_at: Date
  created_at: Date
}
