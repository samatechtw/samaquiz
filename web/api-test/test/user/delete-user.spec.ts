import { testagent, TestAgent } from '../helpers'
import { AppDbResetService } from '@test/shared'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import {
  adminAuthHeader,
  expiredAdminToken,
  expiredUser1Token,
  userAuthHeader,
} from '../helpers'
import { testConfig } from '../test.config'

describe('Delete User', () => {
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
    test('returns 200 when deleting a user', async () => {
      await api
        .delete(`${testEndpoint}/${userId}`)
        .set('Authorization', adminAuth)
        .expect(204)

      await api
        .get(`${testEndpoint}/${userId}`)
        .set('Authorization', adminAuth)
        .expect(404, {
          code: 'None',
          message: 'Not found',
          status: 404,
        })
    })

    test('returns 401 when admin token has expired', async () => {
      await api
        .delete(`${testEndpoint}/${userId}`)
        .set('Authorization', expiredAdminToken)
        .expect(401, {
          code: 'InvalidAuth',
          message: 'Unauthorized',
          status: 401,
        })
    })

    test('returns 404 when deleting a non-existent user', async () => {
      const nonExistId = '870aafc9-36e9-476a-b38c-c1aaaad9d9fe'
      await api
        .delete(`${testEndpoint}/${nonExistId}`)
        .set('Authorization', adminAuth)
        .expect(404, {
          code: 'None',
          message: 'Not found',
          status: 404,
        })
    })
  })

  describe('when requester is User', () => {
    test('returns 200 when deleting own account', async () => {
      await api
        .delete(`${testEndpoint}/${userId}`)
        .set('Authorization', adminAuth)
        .expect(204)

      await api
        .get(`${testEndpoint}/${userId}`)
        .set('Authorization', userAuth)
        .expect(401, {
          code: 'Unauthorized',
          message: 'Unauthorized',
          status: 401,
        })
    })

    test('returns 403 when a user tries to delete another user', async () => {
      await api
        .delete(`${testEndpoint}/${adminId}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    test('returns 401 when token has expired', async () => {
      await api
        .delete(`${testEndpoint}/${userId}`)
        .set('Authorization', expiredUser1Token)
        .expect(401, {
          code: 'InvalidAuth',
          message: 'Unauthorized',
          status: 401,
        })
    })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.delete(`${testEndpoint}/${userId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
