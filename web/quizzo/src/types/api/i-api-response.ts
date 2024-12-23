import { IJsonObject } from '@samatech/fetch-api'

export class ApiResponse<T = IJsonObject> extends Response {
  data!: T
}
