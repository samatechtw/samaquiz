import { IListQuizzesApiRequest, IListQuizzesApiResponse } from '@frontend/types'
import { testagent, TestAgent, adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { AppDbResetService } from '@test/shared'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

const TotalQuizzes = 4

describe('List Quizzes', () => {
  const testEndpoint = '/api/quizzes'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let query: IListQuizzesApiRequest
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
    test('return 200 status code and quizzes with default sorting', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', adminAuth)
        .expect(200)
      const body: IListQuizzesApiResponse = response.body

      expect(body.total).toEqual(TotalQuizzes)
      expect(body.results.length).toEqual(TotalQuizzes)
    })

    test('returns 200 status and quizzes when filtering from 1 to 2', async () => {
      query = { from: 1, to: 2 }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizzesApiResponse = response.body
      const quizzes = body.results

      expect(body.total).toEqual(TotalQuizzes)
      expect(quizzes.length).toEqual(2)

      expect(quizzes[0].id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')
      expect(quizzes[0].questions_order).toHaveLength(4)
      expect(quizzes[1].id).toEqual('efd3863f-a975-4b2d-9b03-eb3fe28410b9')
    })

    test('filters by quiz_types', async () => {
      const response = await api
        .get(testEndpoint)
        .query('types[]=Quiz')
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizzesApiResponse = response.body

      expect(body.total).toEqual(4)
      expect(body.results[0].id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')
      expect(body.results[1].id).toEqual('efd3863f-a975-4b2d-9b03-eb3fe28410b9')
      expect(body.results[2].id).toEqual('5db0613d-0077-48c5-a3f1-f0faefe2a083')
      expect(body.results[3].id).toEqual('7070ba54-6ed7-4916-b3b6-e7251770d0b1')
    })

    test('filters by quiz_type and user-id', async () => {
      const response = await api
        .get(testEndpoint)
        .query('types[]=Quiz&user_id=2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizzesApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')
      expect(body.results[1].id).toEqual('efd3863f-a975-4b2d-9b03-eb3fe28410b9')
      expect(body.results[2].id).toEqual('5db0613d-0077-48c5-a3f1-f0faefe2a083')
    })

    test('filters by user_id', async () => {
      query = { user_id: '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e' }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizzesApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')
      expect(body.results[1].id).toEqual('efd3863f-a975-4b2d-9b03-eb3fe28410b9')
      expect(body.results[2].id).toEqual('5db0613d-0077-48c5-a3f1-f0faefe2a083')
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

      const body: IListQuizzesApiResponse = response.body

      expect(body.total).toEqual(3)
      expect(body.results[0].id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')
      expect(body.results[1].id).toEqual('efd3863f-a975-4b2d-9b03-eb3fe28410b9')
      expect(body.results[2].id).toEqual('5db0613d-0077-48c5-a3f1-f0faefe2a083')
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
