import { FetchApi, FetchRequestConfig } from '@samatech/fetch-api'
import { Ref } from 'vue'
import { ApiResponse } from '@frontend/types'
import { defaultResponseInterceptors } from './default-response-interceptors'
import { transformRequestData } from './api-transforms'

export interface AppApiOptions {
  baseUrl: string
  userToken?: Ref<string | null>
  cors?: boolean
  responseInterceptors?: ((res: Response) => Promise<ApiResponse>)[]
}

export class AppApi extends FetchApi<ApiResponse> {
  userToken?: Ref<string | null>
  cors: boolean

  constructor(options: AppApiOptions) {
    const responseInterceptors = [
      ...(options.responseInterceptors ?? defaultResponseInterceptors),
    ]
    super({
      ...options,
      responseInterceptors,
    })
    this.userToken = options.userToken
    this.cors = options.cors ?? true
  }

  authRequest<T>(config: FetchRequestConfig): Promise<ApiResponse<T>> {
    const { headers, ...rest } = config
    let authHeaders = headers
    if (this.userToken?.value) {
      authHeaders = {
        ...headers,
        Authorization: `Bearer ${this.userToken.value}`,
      }
    }
    return this.request<T>({
      ...rest,
      headers: authHeaders,
    })
  }

  authOptRequest<T>(config: FetchRequestConfig): Promise<ApiResponse<T>> {
    if (this.userToken?.value) {
      return this.authRequest(config)
    } else {
      return this.request(config)
    }
  }

  override async request<T>(config: FetchRequestConfig): Promise<ApiResponse<T>> {
    const configHeaders = config.headers as Record<string, string> | undefined
    const contentType = configHeaders?.['Content-Type'] || 'application/json'
    const finalConfig: FetchRequestConfig = {
      ...config,
      data: transformRequestData(config.data as Record<string, unknown>),
      mode: this.cors ? 'cors' : 'no-cors',
      headers: {
        ...config.headers,
        'Content-Type': contentType,
      },
    }
    return super.request(finalConfig) as unknown as ApiResponse<T>
  }
}
