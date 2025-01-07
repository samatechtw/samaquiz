import { apiConfirmEmail, apiResendConfirmEmail } from '@frontend/api'
import { store } from '@frontend/store'
import { errorToKey } from '@frontend/util/api'
import { jwtDecode, JwtPayload } from 'jwt-decode'
import { reactive } from 'vue'

export interface IConfirmEmailState {
  errorKey: string | undefined
  loading: boolean
  expired: boolean
}

export interface IResendConfirmState {
  errorKey: string | undefined
  loading: boolean
  complete: boolean
}

export const confirmState = reactive<IConfirmEmailState>({
  errorKey: undefined,
  loading: false,
  expired: false,
})

export const resendState = reactive<IResendConfirmState>({
  errorKey: undefined,
  loading: false,
  complete: false,
})

export const confirmEmail = async (code: string) => {
  confirmState.errorKey = undefined
  confirmState.expired = false
  confirmState.loading = true
  let jwt
  try {
    jwt = jwtDecode<JwtPayload>(code)
  } catch (_) {
    // Let the next check set the error
  }
  try {
    if (!jwt?.exp) {
      confirmState.errorKey = 'errors.confirm_email'
      confirmState.loading = false
      return
    }
    if (jwt.exp <= Date.now() / 1000) {
      confirmState.expired = true
      confirmState.loading = false
      return
    }
    await apiConfirmEmail(code)
    store.user.setConfirmed(true)
  } catch (e) {
    confirmState.errorKey = errorToKey(e)
  }
  confirmState.loading = false
}

export const resendConfirm = async () => {
  resendState.errorKey = undefined
  resendState.loading = true
  try {
    await apiResendConfirmEmail()
    resendState.complete = true
  } catch (e) {
    resendState.errorKey = errorToKey(e)
  }
  resendState.loading = false
}
