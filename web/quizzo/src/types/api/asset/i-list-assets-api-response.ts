import { IAssetViewModel } from './i-asset.view-model'

export interface IListAssetsResponse {
  total: number
  total_usage: number
  results: IAssetViewModel[]
}
