import { apiGetUser, apiUpdateUser } from '@frontend/api'
import { store } from '@frontend/store'
import { IUpdateUserApiRequest } from '@frontend/types'
import { errorToKey } from '@frontend/util/api'
import { Ref } from 'vue'

const checkAuthError = (e: unknown): boolean => {
  // TODO -- more reliable way to check user auth/token status
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const status = (e as any).status
  if (status === 404 || status === 401) {
    store.auth.logOut()
    return true
  }
  return false
}

export const getUser = async () => {
  const id = store.auth.userId?.value
  if (id) {
    try {
      const user = await apiGetUser(id)
      if (checkAuthError(user)) {
        return
      }
      store.user.updateUser({
        id: user.id,
        name: user.name,
        description: user.description,
        link: user.link,
        location: user.location,
        avatar: user.avatar,
        email: user.email,
        emailConfirmed: user.email_confirmed,
        userType: user.user_type,
        userStatus: user.user_status,
        createdAt: user.created_at.getTime(),
      })
    } catch (e) {
      checkAuthError(e)
    }
  } else {
    console.error('User ID not found')
  }
}

export const updateUser = async (
  payload: IUpdateUserApiRequest,
  error: Ref<string | undefined>,
) => {
  error.value = undefined
  const id = store.auth.userId?.value
  if (id) {
    try {
      await apiUpdateUser(id, payload)
      store.user.updateUser({
        email: payload.email ?? store.user.email.value,
        name: payload.name ?? store.user.name.value,
        avatar: payload.avatar ?? store.user.avatar.value,
        description: payload.description ?? store.user.description.value,
        link: payload.link ?? store.user.link.value,
        location: payload.location ?? store.user.location.value,
      })
    } catch (e) {
      error.value = errorToKey(e)
    }
  } else {
    console.error('User ID not found')
  }
}
