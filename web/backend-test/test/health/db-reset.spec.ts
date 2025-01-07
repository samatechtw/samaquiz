import { AppDbResetService, testagent, TestAgent } from '../helpers'
import { testConfig } from '../test.config'
import { beforeAll, describe, test } from 'vitest'

describe('Login User', () => {
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  test('reset the database', async () => {
    await dbResetService.resetDb()
  })
})
