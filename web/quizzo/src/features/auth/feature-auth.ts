import { apiLoginUser, apiRegisterUser } from '@frontend/api'
import { computed, ref } from 'vue'
import { errorToKey } from '@frontend/util/api'
import { ILoginUserApiRequest, IRegisterUserApiRequest } from '@frontend/types'
import { store } from '@frontend/store'
import { getUser } from '../user'

export const authErrorKey = ref<string | undefined>()

export const loggedIn = computed(() => {
  return store.auth.loggedIn.value
})

export const validatePassword = (password: string): boolean => {
  if (password.length < 8 || password.length > 50) {
    authErrorKey.value = 'errors.password_length'
    return false
  }
  return true
}

export const registerUser = async (payload: IRegisterUserApiRequest): Promise<void> => {
  authErrorKey.value = undefined
  if (!validatePassword(payload.password)) {
    return
  }
  try {
    await apiRegisterUser(payload)
    await loginUser({ email: payload.email, password: payload.password })
  } catch (e) {
    authErrorKey.value = errorToKey(e)
  }
}

export const loginUser = async (payload: ILoginUserApiRequest): Promise<void> => {
  authErrorKey.value = undefined
  try {
    const authRes = await apiLoginUser(payload)
    if (authRes.auth_token) {
      store.auth.logIn(authRes.auth_token)
      await getUser()
    } else {
      authErrorKey.value = errorToKey(authRes)
    }
  } catch (e) {
    authErrorKey.value = errorToKey(e)
  }
}
