import { IQuizAssetViewModel } from './i-quiz-asset.view-model'

export interface ICreateQuizAssetResponse extends IQuizAssetViewModel {
  signed_url: string
}
