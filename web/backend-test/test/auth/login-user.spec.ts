import { ILoginUserApiRequest, ILoginUserApiResponse } from '@frontend/types'
import { commonRegex } from '@frontend/util/format'
import { omit } from '@frontend/util/misc'
import { AppDbResetService, testagent, TestAgent } from '../helpers'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, expect, test } from 'vitest'

describe('Login User', () => {
  const testEndpoint = '/api/auth/logins'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ILoginUserApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    payload = { email: 'user1@samatech.tw', password: 'password1' }
  })

  describe('when requestor is admin', () => {
    test('logs in user with password', async () => {
      payload = { email: 'admin1@samatech.tw', password: 'admin.password1' }
      const response = await api.post(testEndpoint).send(payload).expect(201)
      const body: ILoginUserApiResponse = response.body

      expect(body.auth_token).toMatch(new RegExp(commonRegex.authToken))
    })
  })

  describe('when requestor is user', () => {
    test('logs in user with password', async () => {
      const response = await api.post(testEndpoint).send(payload).expect(201)
      const body: ILoginUserApiResponse = response.body

      expect(body.auth_token).toMatch(new RegExp(commonRegex.authToken))
    })
  })

  describe('when request is not valid', () => {
    test('when user does not exist', () => {
      payload.email = 'missing@email.com'

      return api.post(testEndpoint).send(payload).expect(401, {
        status: 401,
        message: 'Login failed',
        code: 'InvalidAuth',
      })
    })

    test('when password is incorrect', () => {
      payload.password = '12345678'

      return api.post(testEndpoint).send(payload).expect(401, {
        status: 401,
        message: 'Login failed',
        code: 'InvalidAuth',
      })
    })

    test('when email is not an email', () => {
      payload.password = '1234'

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

    test('when password is missing in payload', () => {
      const noPayload = omit(payload, 'password')

      return api.post(testEndpoint).send(noPayload).expect(400, {
        status: 400,
        message: 'missing field password',
        code: 'InvalidFormData',
      })
    })
  })
})
