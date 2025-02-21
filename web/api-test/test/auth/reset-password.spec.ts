import { IResetPasswordApiRequest } from '@frontend/types'
import { omit } from '@frontend/util/misc'
import { AppDbResetService, testagent, TestAgent } from '../helpers'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, test } from 'vitest'

describe('Reset Password', () => {
  const testEndpoint = '/api/auth/logins/reset-password'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: IResetPasswordApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    payload = { email: 'test@test.com' }
  })

  test('resets password', async () => {
    await api.post(testEndpoint).send(payload).expect(202)
  })

  describe('when request is not valid', () => {
    test('when email is not an email', () => {
      payload.email = '1234'

      return api.post(testEndpoint).send(payload).expect(400, {
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
      })
    })

    test('when email is missing in payload', () => {
      const noEmail = omit(payload, 'email')

      return api.post(testEndpoint).send(noEmail).expect(400, {
        status: 400,
        message: 'missing field email',
        code: 'InvalidFormData',
      })
    })
  })
})
