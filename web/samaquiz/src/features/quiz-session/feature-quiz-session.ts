import { computed, reactive } from 'vue'
import { getUser } from '../user'
import { store } from '@frontend/store'
import {
  IGetQuizSessionApiResponse,
  IListQuizSessionsApiRequest,
  IQuizSessionViewModel,
  ISessionLeader,
} from '@frontend/types'
import { ts } from '../../i18n'
import { errorToKey } from '@frontend/util/api'
import { useLoginRedirect } from '../auth'
import { apiGetSession, apiGetSessionLeaders, apiListQuizSessions } from '@frontend/api'
import { IFeatureParams } from '../i-feature-params'

export interface IListQuizSessionParams extends IFeatureParams {
  quiz_sessions: IQuizSessionViewModel[]
}

export interface IGetQuizSessionParams extends IFeatureParams {
  session: IGetQuizSessionApiResponse | undefined
}

export interface IGetLeadersParams extends IFeatureParams {
  leaders: ISessionLeader[] | undefined
}

export const sessionState: IGetQuizSessionParams = reactive({
  error: undefined,
  loading: false,
  session: undefined,
})

export const quizSession = computed(() => {
  return sessionState.session
})

export const getQuizSession = async (code: string, state: IGetQuizSessionParams) => {
  const { redirectToLogin } = useLoginRedirect()
  state.loading = true
  state.error = undefined
  try {
    await getUser()
    state.session = await apiGetSession(code)
    // Host must be logged in
    const isHost = store.auth.userId.value === state.session.user_id
    if (!store.auth.loggedIn.value && isHost) {
      redirectToLogin({ replace: true })
    }
  } catch (e) {
    if ((e as any).status === 404) {
      state.error = ts('errors.QuizNotFound')
    } else {
      state.error = ts(errorToKey(e))
    }
  }
  state.loading = false
}

export const listQuizSessions = async (
  payload: IListQuizSessionsApiRequest,
  params: IListQuizSessionParams,
) => {
  params.error = undefined
  params.loading = true
  try {
    const res = await apiListQuizSessions(payload)
    params.quiz_sessions = res.results
  } catch (e) {
    params.error = errorToKey(e)
  }
  params.loading = false
}

export const getLeaders = async (
  sessionId: string | undefined,
  params: IGetLeadersParams,
) => {
  if (!sessionId) {
    return
  }
  params.loading = true
  try {
    const res = await apiGetSessionLeaders(sessionId)
    params.leaders = res.leaders
  } catch (e) {
    console.log('Failed to load leaders', e)
  }
  params.loading = false
}
