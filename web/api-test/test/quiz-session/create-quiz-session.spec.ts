import {
  ICreateQuizSessionApiRequest,
  ICreateQuizSessionApiResponse,
  IGetQuizSessionApiResponse,
  IGetQuizApiResponse,
  QuizSessionStatus,
} from '@frontend/types'
import { AppDbResetService } from '@test/shared'
import { commonRegex } from '@frontend/util/format'
import { adminAuthHeader, testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Create Quiz Session', () => {
  const testEndpoint = (quizId: string) => `/api/quizzes/${quizId}/sessions`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateQuizSessionApiRequest
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
      code: 'my-quiz-code',
      host_name: 'My Name',
      host_avatar: 'http://localhost:8080/src/theme/img/cats/cat7.png',
    }
  })

  const verifyQuizSession = async (code: string, auth: string) => {
    const response = await api
      .get(`/api/quiz_sessions/code/${code}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuizSessionApiResponse = response.body
    expect(body.quiz.id).toEqual(quizId)
    expect(body.code).toEqual(payload.code)
    expect(body.host_name).toEqual(payload.host_name)
    expect(body.host_avatar).toEqual(payload.host_avatar)
    expect(body.start_time).toBeNull()
    expect(body.end_time).toBeNull()
    expect(body.question_end_time).toBeNull()
    expect(body.question_index).toBeNull()
    expect(body.question_duration).toEqual(30 * 1000)
    expect(body.status).toEqual(QuizSessionStatus.Ready)
    expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
  }

  test('admin creates quiz session', async () => {
    const response = await api
      .post(testEndpoint(quizId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizSessionApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuizSession(payload.code, adminAuth)

    // Verify added to quiz sessions
    const quizResponse = await api
      .get(`/api/quizzes/${quizId}`)
      .set('Authorization', adminAuth)
      .expect(200)

    const quiz: IGetQuizApiResponse = quizResponse.body
    expect(quiz.sessions.length).toEqual(3)
    const session = quiz.sessions.find((quiz) => quiz.id === body.id)
    expect(session).toBeDefined()
  })

  test('user creates quiz_session', async () => {
    const response = await api
      .post(testEndpoint(quizId))
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizSessionApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuizSession(payload.code, userAuth)
  })

  test('user creates quiz_session', async () => {
    const response = await api
      .post(testEndpoint(quizId))
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizSessionApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuizSession(payload.code, userAuth)
  })

  describe('when request is not valid', () => {
    test('returns 400 code when code length is invalid', async () => {
      // Code too short
      payload.code = ''
      await api
        .post(testEndpoint(quizId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })

      // Code too long
      payload.code = '12345678901234567890123456789012345678901234567890zabcdefghijklm'
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
