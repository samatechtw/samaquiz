import { AppDbResetService, testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, test } from 'vitest'

describe('Resend confirm Email', () => {
  const testEndpoint = '/api/auth/resend-confirm-email'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
  })

  test('return 200 when resend confirm email', () => {
    userAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')
    return api.post(testEndpoint).set('Authorization', userAuth).expect(200)
  })

  test('return 400 when email is already confirmed', () => {
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    return api.post(testEndpoint).set('Authorization', userAuth).expect(400, {
      code: 'AlreadyConfirmed',
      message: 'Resend fail',
      status: 400,
    })
  })

  test('when user is not authorized', () => {
    return api.post(testEndpoint).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
