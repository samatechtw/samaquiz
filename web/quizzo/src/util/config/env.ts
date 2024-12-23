import { ExecEnv } from '@frontend/types'

export const API_HOST = import.meta.env.VITE_API_HOST || ''
export const EXEC_ENV = import.meta.env.VITE_EXEC_ENV || ExecEnv.Development

export const WEB_URL =
  EXEC_ENV === ExecEnv.Production
    ? 'quiz.samatech.tw'
    : `${EXEC_ENV}.quiz.samatech.tw`

// R2 public bucket URL
export const S3_SITE_ASSETS_URL = import.meta.env.VITE_S3_SITE_ASSETS_URL || ''
