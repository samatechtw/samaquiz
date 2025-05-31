import { ApiResponse } from '@frontend/types'
import { FetchRequestConfig } from '@samatech/fetch-api'
import { BasicFetchApi } from './util-fetch-api'

interface IDbResetServiceParams {
  resetBaseUrl: string
  resetUrl: string
}

export interface ApiKeyRequestOptions extends FetchRequestConfig {
  baseUrl: string
  url: string
  timeout?: number
  apiKey: string
  transform?: boolean
}

export const apiRequest = async <T>(
  options: Omit<ApiKeyRequestOptions, 'apiKey'>,
): Promise<ApiResponse<T>> => {
  const { baseUrl, timeout, ...rest } = options
  const api = new BasicFetchApi({ baseUrl, timeout })
  try {
    return await api.request<T>(rest)
  } catch (e) {
    const err = JSON.stringify(e)
    throw new Error(
      `API request failed, options=${JSON.stringify(options)}, error=${err}`,
    )
  }
}

export class DbResetService {
  private resetBaseUrl: string
  private resetUrl: string

  constructor(options: IDbResetServiceParams) {
    this.resetUrl = options.resetUrl
    this.resetBaseUrl = options.resetBaseUrl
  }

  resetDb() {
    return apiRequest({
      method: 'POST',
      baseUrl: this.resetBaseUrl,
      url: this.resetUrl,
      timeout: 10000,
    })
  }
}

export class AppDbResetService extends DbResetService {
  constructor(testHelperUrl: string) {
    super({
      resetBaseUrl: testHelperUrl,
      resetUrl: '/actions/reset/db-app',
    })
  }
}
