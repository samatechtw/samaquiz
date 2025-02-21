import { IGetUserApiResponse, UserType } from '@frontend/types'
import { testagent, TestAgent, AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import {
  adminAuthHeader,
  expiredAdminToken,
  expiredUser1Token,
  userAuthHeader,
} from '../helpers'
import { testConfig } from '../test.config'

describe('Get User', () => {
  const testEndpoint = '/api/users'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  const adminId = '083fb9af-a2fd-41b8-bfa2-4747cc87b604'
  const userId = '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e'

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader(userId)
  })

  describe('when requester is Admin', () => {
    test('returns 200 and message when get an admin user', async () => {
      const response = await api
        .get(`${testEndpoint}/${adminId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetUserApiResponse = response.body

      expect(body.id).toEqual(adminId)
      expect(body.name).toEqual('Admin')
      expect(body.description).toEqual('SamaTech Admin')
      expect(body.link).toEqual('https://samatech.tw')
      expect(body.location).toEqual('Hong Kong')
      expect(body.email).toEqual('admin1@samatech.tw')
      expect(body.user_type).toEqual('Admin')
      expect(body.email_confirmed).toEqual(true)
    })

    test('returns 200 and message when get an user user', async () => {
      const response = await api
        .get(`${testEndpoint}/${userId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetUserApiResponse = response.body

      expect(body.id).toEqual(userId)
      expect(body.name).toEqual('user1@samatech.tw')
      expect(body.description).toEqual('First User')
      expect(body.link).toEqual(
        'https://samatech.tw/user/2213d9fc-3693-47ed-a495-cd5e7fc6dd0e',
      )
      expect(body.location).toEqual('Japan')
      expect(body.email).toEqual('user1@samatech.tw')
      expect(body.user_type).toEqual(UserType.User)
      expect(body.email_confirmed).toEqual(true)
    })

    test('returns 401 when admin token has expired', async () => {
      await api
        .get(`${testEndpoint}/${userId}`)
        .set('Authorization', expiredAdminToken)
        .expect(401, {
          code: 'InvalidAuth',
          message: 'Unauthorized',
          status: 401,
        })
    })
  })

  describe('when requester is User', () => {
    test('returns 200 and message when get an user user', async () => {
      const response = await api
        .get(`${testEndpoint}/${userId}`)
        .set('Authorization', userAuth)
        .expect(200)

      const body: IGetUserApiResponse = response.body

      expect(body.id).toEqual(userId)
      expect(body.email).toEqual('user1@samatech.tw')
      expect(body.user_type).toEqual(UserType.User)
      expect(body.email_confirmed).toEqual(true)
    })

    test('returns 401 when user token has expired', async () => {
      await api
        .get(`${testEndpoint}/${userId}`)
        .set('Authorization', expiredUser1Token)
        .expect(401, {
          code: 'InvalidAuth',
          message: 'Unauthorized',
          status: 401,
        })
    })

    test('returns 403 error when user user gets another user', async () => {
      const otherUserId = '0c069253-e45d-487c-b7c0-cbe467c33a10'
      await api
        .get(`${testEndpoint}/${otherUserId}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    test('returns 403 error when user user gets an admin user', async () => {
      await api
        .get(`${testEndpoint}/${adminId}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.get(`${testEndpoint}/${adminId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })

  test('returns 404 when user does not exist', async () => {
    const nonExistId = '870aafc9-36e9-476a-b38c-c1aaaad9d9fe'
    await api
      .get(`${testEndpoint}/${nonExistId}`)
      .set('Authorization', adminAuth)
      .expect(404, {
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })
})
