// Miscellaneous app state
// Includes state used for app control flow or state without an obvious category
import { LocalStoragePlugin, useModule } from '@samatech/vue-store'

export interface IMiscState {
  cookiesAccepted: boolean
  locale: string
}

const getters = (state: IMiscState) => ({
  cookiesAccepted: () => state.cookiesAccepted,
})

const mutations = (state: IMiscState) => ({
  setCookiesAccepted(accepted: boolean) {
    state.cookiesAccepted = accepted
  },
  setLocale: (locale: string) => {
    state.locale = locale
  },
})

export const miscModule = useModule<
  IMiscState,
  ReturnType<typeof getters>,
  ReturnType<typeof mutations>
>({
  name: 'misc-store',
  version: 4,
  stateInit: () => ({
    cookiesAccepted: false,
    locale: 'en',
  }),
  getters,
  mutations,
  plugins: [LocalStoragePlugin],
})
