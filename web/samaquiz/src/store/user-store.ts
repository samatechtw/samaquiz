import { UserType } from '@frontend/types'
import { defaultCat } from '@frontend/util/ui'
import { LocalStoragePlugin, useModule } from '@samatech/vue-store'

interface UserState {
  id: string
  name: string
  description: string
  link: string
  email: string
  location: string
  avatar: string
  userType: string
  userStatus: string
  emailConfirmed: boolean
  createdAt: number | null
}

const userInit = (): UserState => ({
  id: '',
  name: '',
  description: '',
  link: '',
  location: '',
  avatar: '',
  email: '',
  emailConfirmed: false,
  userType: '',
  userStatus: '',
  createdAt: null,
})

export const userGetters = (state: UserState) => ({
  isAdmin: () => {
    return state.userType === UserType.Admin
  },
  getAvatar: () => {
    return state.avatar || defaultCat
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
  version: 4,
  stateInit: userInit,
  mutations: userMutations,
  getters: userGetters,
  plugins: [LocalStoragePlugin],
})

export type UserModule = typeof userModule
