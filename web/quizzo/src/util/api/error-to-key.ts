import { IApiError } from '@frontend/types'

export const errorToKey = (apiError: unknown): string => {
  if (!apiError) {
    return 'errors.None'
  }
  const err = apiError as IApiError
  const { code, message } = err
  if (code) {
    return `errors.${err.code}`
  } else if (message) {
    return message
  }
  return 'errors.None'
}
