import { Reactive } from 'vue'

export type IFeatureParams = Reactive<{
  error: string | undefined
  loading: boolean
}>
