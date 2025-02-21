import { IRegisterUserApiRequest, IRegisterUserApiResponse } from '@frontend/types'
import { omit } from '@frontend/util/misc'
import { commonRegex } from '@frontend/util/format'
import { AppDbResetService, testagent, TestAgent } from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Register User', () => {
  const testEndpoint = '/api/users/registrations'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: IRegisterUserApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    payload = {
      email: 'test@test.com',
      password: '12345678',
    }
  })

  test('registers user', async () => {
    const response = await api.post(testEndpoint).send(payload).expect(201)
    const body: IRegisterUserApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
  })

  describe('when request is not valid', () => {
    test('returns 400 code when password length is invalid', async () => {
      // Password too short
      payload.password = '1234'
      await api.post(testEndpoint).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
      })

      // Password too long
      payload.password = '12345678901234567890123456789012345678901234567890z'
      return api.post(testEndpoint).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
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

    test('returns 400 code when email already exists', async () => {
      // Try to create a user with the same email as a user from fixtures
      payload = {
        email: 'user2@samatech.tw',
        password: '12345678',
      }
      await api.post(testEndpoint).send(payload).expect({
        code: 'UserExists',
        message: 'User already exists',
        status: 400,
      })
    })

    test('returns 400 code when email already exists', () => {
      // Try to create a user with the same email as a user from fixtures
      payload.email = 'admin1@samatech.tw'
      return api.post(testEndpoint).send(payload).expect({
        code: 'UserExists',
        message: 'User already exists',
        status: 400,
      })
    })
  })
})
