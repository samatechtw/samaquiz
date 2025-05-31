import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { beforeAll, describe, test } from 'vitest'

describe('Login User', () => {
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService

  beforeAll(() => {
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  test('reset the database', async () => {
    await dbResetService.resetDb()
  })
})
