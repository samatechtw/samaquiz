import { IGetUserApiResponse } from '@frontend/types'
import { testagent, TestAgent, generateConfirmToken } from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, AppDbResetService } from '../helpers'

describe('Confirm Email', () => {
  const testEndpoint = '/api/auth/confirm-email'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
  })

  test('return 200 when email confirmation succeeds', async () => {
    // Generate a mock JWT using user3's ID
    const userId = '028ba9f2-f360-423b-83b6-44863b69e211'
    const payload = {
      code: generateConfirmToken(userId),
    }
    await api.post(testEndpoint).send(payload).expect(200)

    // Get User3
    const response = await api
      .get(`/api/users/028ba9f2-f360-423b-83b6-44863b69e211`)
      .set('Authorization', adminAuth)
      .expect(200)

    const body: IGetUserApiResponse = response.body

    expect(body.email_confirmed).toEqual(true)
  })

  test('return an error when an expired code is provided', async () => {
    // Generate a mock JWT using user3's ID
    const userId = '028ba9f2-f360-423b-83b6-44863b69e211'
    const payload = {
      code: generateConfirmToken(userId, '-90s'),
    }

    return api.post(testEndpoint).send(payload).expect(400, {
      status: 400,
      message: 'Confirm token decode error: ExpiredSignature',
      code: 'ConfirmExpired',
    })
  })

  test('return an error when the user does not exist', async () => {
    // Generate a mock JWT using user3's ID
    const userId = '6bf84234-1939-4a49-bfc7-ee00f6b6d694'
    const payload = {
      code: generateConfirmToken(userId),
    }

    return api.post(testEndpoint).send(payload).expect(400, {
      status: 400,
      message: 'User not found',
      code: 'None',
    })
  })

  test('return an error when an invalid code is provided', async () => {
    return api.post(testEndpoint).send({ code: 'invalid code' }).expect(400, {
      status: 400,
      message: 'Confirm token decode error: InvalidToken',
      code: 'None',
    })
  })
})
