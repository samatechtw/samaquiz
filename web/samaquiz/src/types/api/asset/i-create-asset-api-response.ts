import { IAssetViewModel } from './i-asset.view-model'

export interface ICreateAssetResponse extends IAssetViewModel {
  signed_url: string
}
