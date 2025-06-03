import { store } from '@frontend/store'
import { ApiResponse } from '@frontend/types'
import { API_HOST, WS_PREFIX, URL_PREFIX } from '@frontend/util/config'
import { AppApi } from '@frontend/util/api'

export const API_URL = `${URL_PREFIX}${API_HOST}/api/`
export const WS_URL = `${WS_PREFIX}${API_HOST}/api/ws`

export const setupApiInterceptors = (api: AppApi) => {
  api.interceptResponse(async (res: Response): Promise<ApiResponse> => {
    if (res.status === 401) {
      if (
        // Don't redirect if we're already on the login page
        location.pathname !== '/login' &&
        // Login 401 is handled within the Login and ResetPassword component
        !res.url.match(/^.*?\/auth\/logins(\/passwords)?$/) &&
        // External 401 response should not trigger logout
        res.url.startsWith(API_URL)
      ) {
        // Can't use redirectToLogin outside a component, so do it manually
        store.auth.setLoginRedirect(location.pathname)
        store.auth.logOut()
      }

      throw res
    } else if (res.status === 405) {
      throw res
    }
    return res as ApiResponse
  })
  return api
}

export const rootApi = setupApiInterceptors(
  new AppApi({
    baseUrl: API_URL,
    userToken: store.auth.token,
  }),
)
