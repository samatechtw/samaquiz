import { IGetQuestionApiResponse } from '@frontend/types'
import { IFeatureParams } from '../i-feature-params'
import { errorToKey } from '@frontend/util/api'
import { apiGetQuestion } from '@frontend/api'

export interface IGetQuestionParams extends IFeatureParams {
  question: IGetQuestionApiResponse | undefined
}

export const getQuestion = async (
  questionId: string,
  questionState: IGetQuestionParams,
) => {
  questionState.error = undefined
  questionState.loading = true
  try {
    const res = await apiGetQuestion(questionId)
    questionState.question = res
  } catch (e) {
    questionState.error = errorToKey(e)
  }
  questionState.loading = false
}
