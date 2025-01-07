import { ISortOption } from '../shared'
import { AssetContentType } from './asset-content-type'
import { AssetState } from './asset-state'

export interface IListAssetsRequest extends ISortOption {
  user_id?: string
  project_id?: string
  content_type?: AssetContentType
  state?: AssetState
  from?: number
  to?: number
}
