import { apiGetQuiz, apiListQuizzes } from '@frontend/api'
import {
  IGetQuizApiResponse,
  IListQuizzesApiRequest,
  IQuizViewModel,
} from '@frontend/types'
import { errorToKey } from '@frontend/util/api'
import { IFeatureParams } from '../i-feature-params'
import { reactive } from 'vue'

export interface IListQuizParams extends IFeatureParams {
  quizzes: IQuizViewModel[]
}

export enum NewSessionStatus {
  None,
  Show,
  Created,
}

export interface IGetQuizParams extends IFeatureParams {
  quiz: IGetQuizApiResponse | undefined
  newSession: NewSessionStatus
}

export const quizState: IGetQuizParams = reactive({
  error: undefined,
  loading: false,
  quiz: undefined,
  newSession: NewSessionStatus.None,
})

export const listQuizzes = async (
  payload: IListQuizzesApiRequest,
  params: IListQuizParams,
) => {
  params.error = undefined
  params.loading = true
  try {
    const res = await apiListQuizzes(payload)
    params.quizzes = res.results
  } catch (e) {
    params.error = errorToKey(e)
  }
  params.loading = false
}

export const getQuiz = async (id: string) => {
  quizState.error = undefined
  quizState.loading = true
  try {
    const res = await apiGetQuiz(id)
    quizState.quiz = res
  } catch (e) {
    quizState.error = errorToKey(e)
  }
  quizState.loading = false
}
