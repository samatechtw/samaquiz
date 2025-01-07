import { ExecEnv } from '@frontend/types'

export const API_HOST = import.meta.env.VITE_API_HOST || ''
export const EXEC_ENV = import.meta.env.VITE_EXEC_ENV || ExecEnv.Development

export const URL_PREFIX = EXEC_ENV === ExecEnv.Development ? 'http://' : 'https://'

export const WEB_URL =
  URL_PREFIX +
  ({
    [ExecEnv.Production]: 'quiz.sampullman.com',
    [ExecEnv.Staging]: 'stg.quiz.sampullman.com',
    [ExecEnv.Development]: 'localhost:8080',
  }[EXEC_ENV] ?? `${EXEC_ENV}.quiz.sampullman.com`)

// R2 public bucket URL
export const S3_SITE_ASSETS_URL = import.meta.env.VITE_S3_SITE_ASSETS_URL || ''
