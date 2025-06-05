import { ExecEnv } from '@frontend/types'

export const API_HOST = import.meta.env.VITE_API_HOST || ''
export const EXEC_ENV = import.meta.env.VITE_EXEC_ENV || ExecEnv.Development

const devOrCI = ExecEnv.Development || ExecEnv.CI

export const URL_PREFIX = EXEC_ENV === devOrCI ? 'http://' : 'https://'
export const WS_PREFIX = devOrCI ? 'ws://' : 'wss://'

export const WEB_URL =
  URL_PREFIX +
  ({
    [ExecEnv.Production]: 'quiz.sampullman.com',
    [ExecEnv.Staging]: 'stg.quiz.sampullman.com',
    [ExecEnv.Development]: 'localhost:8080',
    [ExecEnv.CI]: 'localhost:8081',
  }[EXEC_ENV] ?? `${EXEC_ENV}.quiz.sampullman.com`)

// R2 public bucket URL
export const S3_QUIZ_ASSETS_URL = import.meta.env.VITE_S3_QUIZ_ASSETS_URL || ''
