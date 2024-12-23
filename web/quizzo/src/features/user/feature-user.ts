import { apiGetUser, apiUpdateUser } from '@frontend/api'
import { store } from '@frontend/store'
import { IUpdateUserApiRequest } from '@frontend/types'
import { minidenticon } from '@frontend/util/misc'
import { errorToKey } from '@frontend/util/api'
import { Ref } from 'vue'

export const getUser = async () => {
  const id = store.auth.userId?.value
  if (id) {
    try {
      const user = await apiGetUser(id)
      store.user.updateUser({
        id: user.id,
        name: user.name,
        description: user.description,
        link: user.link,
        location: user.location,
        email: user.email,
        emailConfirmed: user.email_confirmed,
        userType: user.user_type,
        userStatus: user.user_status,
        createdAt: user.created_at.getTime(),
      })
      if (!store.user.identiconAvatar.value) {
        store.user.updateUser({ identiconAvatar: minidenticon(user.email) })
      }
    } catch (e) {
      // TODO -- more reliable way to check user auth/token status
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      if ((e as any).statusCode === 404) {
        store.auth.logOut()
      }
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
