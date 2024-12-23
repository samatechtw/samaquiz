import { IGetQuestionApiResponse, QuestionType } from '@frontend/types'
import { testagent, TestAgent, AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { commonRegex } from '@frontend/util/format'

describe('Get Question', () => {
  const testEndpoint = (quizId: string, questionId: string) =>
    `/api/quizzes/${quizId}/questions/${questionId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let quizId: string
  let questionId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
    questionId = '11354d45-903d-4493-9b96-5f07497b01e1'
  })

  describe('when requester is Admin', () => {
    test('returns 200 and question', async () => {
      const response = await api
        .get(testEndpoint(quizId, questionId))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuestionApiResponse = response.body
      expect(body.id).toEqual(questionId)
      expect(body.quiz_id).toEqual(quizId)
      expect(body.text).toEqual('Quiz 1 Question 1')
      expect(body.question_type).toEqual(QuestionType.MultipleChoice)
      expect(body.answers).toHaveLength(4)
      expect(body.answers_order).toHaveLength(4)
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns question with relations', async () => {
      const response = await api
        .get(testEndpoint(quizId, questionId))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuestionApiResponse = response.body
      expect(body.id).toEqual(questionId)
      const quest = body.answers

      expect(quest.length).toEqual(4)
      expect(quest[0].id).toEqual('f424b181-ec35-4a3b-9b81-d467cd0cbe7b')
      expect(quest[0].text).toBeDefined()
      expect(quest[0].is_correct).toBe(true)

      expect(quest[1].id).toEqual('66f41b1d-7777-4045-8ecf-30519a3a0a30')
      expect(quest[1].text).toBeDefined()
      expect(quest[1].is_correct).toBe(false)

      expect(quest[2].id).toEqual('869563bf-63b0-47d4-812e-be31e73fe79a')
      expect(quest[2].text).toBeDefined()
      expect(quest[2].is_correct).toBe(false)

      expect(quest[3].id).toEqual('a314a297-5d3c-42f3-a7a7-7d8726aafd1f')
      expect(quest[3].text).toBeDefined()
      expect(quest[3].is_correct).toBe(false)
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    })

    test('returns 200 and question when getting own question', async () => {
      const response = await api
        .get(testEndpoint(quizId, questionId))
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetQuestionApiResponse = response.body

      expect(body.id).toEqual(questionId)
      expect(body.quiz_id).toEqual(quizId)
      expect(body.text).toEqual('Quiz 1 Question 1')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns 403 error when user gets another user question', async () => {
      userAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')

      await api
        .get(testEndpoint(quizId, questionId))
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 401 error when anonymous user gets a question', async () => {
    await api.get(testEndpoint(quizId, questionId)).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })

  test('returns 404 when question does not belong to quiz', () => {
    quizId = '7070ba54-6ed7-4916-b3b6-e7251770d0b1'

    return api
      .get(testEndpoint(quizId, questionId))
      .set('Authorization', adminAuth)
      .expect({
        code: 'None',
        message: 'Question not found in quiz',
        status: 404,
      })
  })

  test('returns 404 when question does not exist', async () => {
    const nonExistId = '870aafc9-36e9-476a-b38c-c1aaaad9d9fe'
    await api
      .get(testEndpoint(quizId, nonExistId))
      .set('Authorization', adminAuth)
      .expect(404, {
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })
})
