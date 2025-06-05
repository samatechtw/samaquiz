import { IQuizAssetViewModel } from './i-quiz-asset.view-model'

export interface IListQuizAssetsResponse {
  total: number
  total_usage: number
  results: IQuizAssetViewModel[]
}
