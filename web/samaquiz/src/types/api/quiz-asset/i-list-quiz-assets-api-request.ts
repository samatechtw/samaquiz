import { ISortOption } from '../shared'
import { AssetContentType } from './asset-content-type'
import { QuizAssetState } from './quiz-asset-state'

export interface IListQuizAssetsRequest extends ISortOption {
  quiz_id?: string
  user_id?: string
  content_type?: AssetContentType
  state?: QuizAssetState
  from?: number
  to?: number
}
