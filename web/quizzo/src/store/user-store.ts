import { UserType } from '@frontend/types'
import { minidenticon } from '@frontend/util/misc'
import { LocalStoragePlugin, useModule } from '@samatech/vue-store'

interface UserState {
  id: string
  name: string
  description: string
  link: string
  email: string
  location: string
  userType: string
  userStatus: string
  emailConfirmed: boolean
  createdAt: number | null
  identiconAvatar: string | null
}

const userInit = (): UserState => ({
  id: '',
  name: '',
  description: '',
  link: '',
  location: '',
  email: '',
  emailConfirmed: false,
  userType: '',
  userStatus: '',
  createdAt: null,
  identiconAvatar: null,
})

export const userGetters = (state: UserState) => ({
  isAdmin: () => {
    return state.userType === UserType.Admin
  },
})

const userMutations = (user: UserState) => ({
  resetUser: () => {
    Object.assign(user, userInit())
  },
  updateUser: <Key extends keyof UserState>(userData: Pick<UserState, Key>) => {
    for (const key of Object.keys(userData)) {
      const k = key as keyof typeof userData
      user[k] = userData[k]
    }
  },
  updateEmail: (email: string) => {
    user.email = email
    user.identiconAvatar = minidenticon(email)
    user.emailConfirmed = false
  },
  setConfirmed: (confirmed: boolean) => {
    user.emailConfirmed = confirmed
  },
})

export const userModule = useModule<
  UserState,
  ReturnType<typeof userGetters>,
  ReturnType<typeof userMutations>
>({
  name: 'user',
  version: 2,
  stateInit: userInit,
  mutations: userMutations,
  getters: userGetters,
  plugins: [LocalStoragePlugin],
})

export type UserModule = typeof userModule
