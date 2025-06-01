import {
  IListQuizSessionsApiRequest,
  IListQuizSessionsApiResponse,
} from '@frontend/types'
import { testagent, TestAgent, adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { AppDbResetService } from '@test/shared'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

const TotalQuizSessions = 7

describe('List Quiz Sessions', () => {
  const testEndpoint = '/api/quiz_sessions'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let query: IListQuizSessionsApiRequest
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

  describe('when requester is Admin', () => {
    test('return 200 status code and quiz sessions with default sorting', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', adminAuth)
        .expect(200)
      const body: IListQuizSessionsApiResponse = response.body

      expect(body.total).toEqual(TotalQuizSessions)
      expect(body.results.length).toEqual(TotalQuizSessions)
    })

    test('returns 200 status and quiz sessions when filtering from 1 to 2', async () => {
      query = { from: 1, to: 2 }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizSessionsApiResponse = response.body
      const sessions = body.results

      expect(body.total).toEqual(TotalQuizSessions)
      expect(sessions.length).toEqual(2)

      expect(body.results[0].id).toEqual('ce782f8a-01bb-4228-9e50-4db0248f14cd')
      expect(body.results[1].id).toEqual('dab9771d-e818-435f-98de-6a734189ba7d')
    })

    test('filters by user_id', async () => {
      const userId = '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e'
      query = { user_id: userId }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizSessionsApiResponse = response.body

      expect(body.total).toEqual(7)
      expect(body.results.length).toEqual(7)
      expect(body.results[0].id).toEqual('ce782f8a-01bb-4228-9e50-4db0248f14cd')
      expect(body.results[1].id).toEqual('dab9771d-e818-435f-98de-6a734189ba7d')
      expect(body.results[2].id).toEqual('ffb8d80d-086f-4ce3-999b-ae9842afb564')
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    })

    test('filters by own user_id', async () => {
      query = { user_id: '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)

      const body: IListQuizSessionsApiResponse = response.body

      expect(body.total).toEqual(7)
      expect(body.results.length).toEqual(7)
    })

    test('filters by other user_id', async () => {
      query = { user_id: '028ba9f2-f360-423b-83b6-44863b69e211' }
      await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    test('omits user_id in query', async () => {
      await api.get(testEndpoint).set('Authorization', userAuth).expect(403, {
        code: 'None',
        message: 'Forbidden',
        status: 403,
      })
    })
  })

  describe('when requester is Anonymous', () => {
    test('returns 401 error', async () => {
      await api.get(testEndpoint).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
