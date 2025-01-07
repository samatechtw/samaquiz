import { IUpdatePasswordApiRequest } from '@frontend/types'
import {
  testagent,
  TestAgent,
  adminAuthHeader,
  userAuthHeader,
  AppDbResetService,
} from '../helpers'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, test } from 'vitest'

describe('Update Password endpoint', () => {
  let api: TestAgent
  let testHelperApiUrl: string
  let dbStateResetService: AppDbResetService
  let testEndpoint: string
  let userId: string
  let authHeader: string
  let payload: IUpdatePasswordApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbStateResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbStateResetService.resetDb()
    testEndpoint = '/api/auth/logins/passwords'
    userId = '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e'
    authHeader = userAuthHeader(userId)
    payload = { password: 'some-password' }
  })

  describe('when request is valid', () => {
    test('updates password when user is User and logs in with new password', async () => {
      await api
        .patch(testEndpoint)
        .set('Authorization', authHeader)
        .send(payload)
        .expect(204)

      await api
        .post('/api/auth/logins')
        .send({ email: 'user1@samatech.tw', password: payload.password })
        .expect(201)
    })

    test('updates password when user is Admin', async () => {
      authHeader = adminAuthHeader()
      await api
        .patch(testEndpoint)
        .set('Authorization', authHeader)
        .send(payload)
        .expect(204)

      await api
        .post('/api/auth/logins')
        .send({ email: 'admin1@samatech.tw', password: payload.password })
        .expect(201)
    })
  })

  describe('when request is not valid returns error', () => {
    test('when authorization header is missing', () => {
      return api.patch(testEndpoint).send(payload).expect(401)
    })

    test('when user does not exist', () => {
      authHeader = userAuthHeader('cd824b31-afe3-4ef2-a8cd-8cc2c5ff75f9')
      return api.patch(testEndpoint).send(payload).expect(401)
    })

    test('when password is missing in payload', () => {
      return api
        .patch(testEndpoint)
        .set('Authorization', authHeader)
        .send({})
        .expect(400, {
          status: 400,
          message: 'missing field password',
          code: 'InvalidFormData',
        })
    })

    test('when payload contains unknown prop', () => {
      return api
        .patch(testEndpoint)
        .set('Authorization', authHeader)
        .send({ password: 'some34.password234', unknown_prop: 'some value' })
        .expect(400, {
          status: 400,
          // TODO -- clean this up
          message:
            'Failed to deserialize the JSON body into the target type: unknown_prop: unknown field `unknown_prop`, expected `password` at line 1 column 47',
          code: 'InvalidFormData',
        })
    })
  })
})
