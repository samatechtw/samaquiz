import { AppDbResetService } from '@test/shared'

export const API_TEST_HELPER_URL = 'http://localhost:3001'
export const JOB_TEST_HELPER_URL = 'http://localhost:3334'
export const PLATFORM_API_URL = 'http://localhost:3000'

export const resetDb = async () => {
  await new AppDbResetService(API_TEST_HELPER_URL).resetDb()
}
