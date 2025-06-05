import {
  ICreateQuestionApiRequest,
  ICreateQuestionApiResponse,
  IGetQuestionApiResponse,
  IGetQuizApiResponse,
  QuestionType,
} from '@frontend/types'
import { AppDbResetService } from '@test/shared'
import { commonRegex } from '@frontend/util/format'
import { adminAuthHeader, testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Create Question', () => {
  const testEndpoint = (quizId: string) => `/api/quizzes/${quizId}/questions`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateQuestionApiRequest
  let adminAuth: string
  let userAuth: string
  let quizId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    quizId = 'efd3863f-a975-4b2d-9b03-eb3fe28410b9'
    payload = {
      text: 'My New Question',
    }
  })

  const verifyQuestion = async (id: string, auth: string) => {
    const response = await api
      .get(`/api/questions/${id}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuestionApiResponse = response.body
    expect(body.text).toEqual(payload.text)
    expect(body.question_type).toEqual(QuestionType.MultipleChoice)
    expect(body.answers).toEqual([])
    expect(body.answers_order).toEqual([])
    expect(body.asset_url).toEqual('')
    expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
  }

  test('admin creates question', async () => {
    const response = await api
      .post(testEndpoint(quizId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuestionApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuestion(body.id, adminAuth)

    // Verify id added to questions_order
    const quizResponse = await api
      .get(`/api/quizzes/${quizId}`)
      .set('Authorization', adminAuth)
      .expect(200)

    const quiz: IGetQuizApiResponse = quizResponse.body
    expect(quiz.questions.length).toEqual(3)
    expect(quiz.questions_order.length).toEqual(3)
    expect(quiz.questions_order[2]).toEqual(body.id)
  })

  test('user creates question', async () => {
    const response = await api
      .post(testEndpoint(quizId))
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuestionApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuestion(body.id, userAuth)
  })

  describe('when request is not valid', () => {
    test('returns 400 code when text length is invalid', async () => {
      // Text too short
      payload.text = 'a'
      await api
        .post(testEndpoint(quizId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })

      // Text too long
      payload.text =
        '12345678901234567890123456789012345678901234567890zabcdefghijklm' +
        'a'.repeat(1000)
      return api
        .post(testEndpoint(quizId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('returns 401 when user is not authorized', async () => {
      await api.post(testEndpoint(quizId)).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
