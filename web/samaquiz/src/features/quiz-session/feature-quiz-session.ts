import { ref } from 'vue'
import { getUser } from '../user'
import { store } from '@frontend/store'
import { IGetQuizSessionApiResponse } from '@frontend/types'
import { ts } from '../../i18n'
import { errorToKey } from '@frontend/util/api'
import { useLoginRedirect } from '../auth'
import { apiGetSession } from '@frontend/api'

export const loadingSession = ref(false)
export const sessionError = ref()
export const quizSession = ref<IGetQuizSessionApiResponse>()

export const getQuizSession = async (code: string) => {
  const { redirectToLogin } = useLoginRedirect()
  loadingSession.value = true
  sessionError.value = undefined
  try {
    await getUser()
    quizSession.value = await apiGetSession(code)
    // Host must be logged in
    const isHost = store.auth.userId.value === quizSession.value.user_id
    if (!store.auth.loggedIn.value && isHost) {
      redirectToLogin({ replace: true })
    }
  } catch (e) {
    if ((e as any).status === 404) {
      sessionError.value = ts('errors.QuizNotFound')
    } else {
      sessionError.value = ts(errorToKey(e))
    }
  }
  loadingSession.value = false
}
