import { IGetQuizApiResponse, QuizType } from '@frontend/types'
import { testagent, TestAgent, AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { commonRegex } from '@frontend/util/format'

describe('Get Quiz', () => {
  const testEndpoint = '/api/quizzes'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let quizId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
  })

  describe('when requester is Admin', () => {
    test('returns 200 and quiz', async () => {
      const response = await api
        .get(`${testEndpoint}/${quizId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuizApiResponse = response.body
      expect(body.id).toEqual(quizId)
      expect(body.user_id).toEqual('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
      expect(body.title).toEqual('My First Quiz')
      expect(body.description).toEqual('A quiz about my favorite things')
      expect(body.quiz_type).toEqual(QuizType.Quiz)
      expect(body.questions).toHaveLength(4)
      expect(body.questions_order).toHaveLength(4)
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns quiz with relations', async () => {
      quizId = 'efd3863f-a975-4b2d-9b03-eb3fe28410b9'

      const response = await api
        .get(`${testEndpoint}/${quizId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuizApiResponse = response.body
      expect(body.id).toEqual(quizId)
      const quest = body.questions
      expect(quest.length).toEqual(2)
      expect(quest[0].id).toEqual('a15ff0f0-8825-4b64-8a16-2add5747bf42')
      expect(quest[0].text).toBeDefined()
      expect(quest[0].question_type).toBeDefined()
      expect(quest[0].answers_order).toEqual([])

      expect(quest[1].id).toEqual('d8e51677-32f8-4d71-aa68-e7ea3a0e8135')
      expect(quest[1].text).toBeDefined()
      expect(quest[1].question_type).toBeDefined()
      expect(quest[1].answers_order).toEqual([])
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    })

    test('returns 200 and quiz when getting own quiz', async () => {
      const response = await api
        .get(`${testEndpoint}/${quizId}`)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetQuizApiResponse = response.body

      expect(body.id).toEqual(quizId)
      expect(body.user_id).toEqual('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
      expect(body.title).toEqual('My First Quiz')
      expect(body.description).toEqual('A quiz about my favorite things')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns 403 error when user gets another user quiz', async () => {
      const otherQuizId = '7070ba54-6ed7-4916-b3b6-e7251770d0b1'
      await api
        .get(`${testEndpoint}/${otherQuizId}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 401 error when anonymous user gets a quiz', async () => {
    await api.get(`${testEndpoint}/${quizId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })

  test('returns 404 when quiz does not exist', async () => {
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
