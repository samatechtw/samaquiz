import { IGetQuestionApiResponse, IUpdateAnswerApiRequest } from '@frontend/types'
import {
  testagent,
  TestAgent,
  adminAuthHeader,
  userAuthHeader,
  AppDbResetService,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { commonRegex } from '@frontend/util/format'

describe('Update Answer', () => {
  const testEndpoint = (quizId: string, questionId: string, answerId: string) =>
    `/api/quizzes/${quizId}/questions/${questionId}/answers/${answerId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let quizId: string
  let questionId: string
  let answerId: string
  let payload: IUpdateAnswerApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
    questionId = '813a13c9-4562-4fa3-8d23-f46a079a57de'
    answerId = 'd06315f6-6304-45c0-9020-983b232e4701'
    payload = { text: 'Good Answer.' }
  })

  const verifyAnswer = async (id: string, auth: string) => {
    const response = await api
      .get(`/api/quizzes/${quizId}/questions/${questionId}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuestionApiResponse = response.body
    const answer = body.answers.find((a) => a.id === id)
    expect(answer).toBeDefined()
    if (payload.text) {
      expect(answer?.text).toEqual(payload.text)
    }
    if (payload.is_correct !== undefined) {
      expect(answer?.is_correct).toEqual(payload.is_correct)
    }
    if (payload.points !== undefined) {
      expect(answer?.points).toEqual(payload.points)
    }
    expect(answer?.updated_at).toMatch(new RegExp(commonRegex.date))
    expect(answer?.created_at).toMatch(new RegExp(commonRegex.date))

    if (payload.is_correct) {
      for (const answer of body.answers) {
        if (answer.id !== id) {
          expect(answer.is_correct).toBe(false)
        }
      }
    }
  }

  describe('when requestor is Admin', () => {
    test('return 200 when updating answer text', async () => {
      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyAnswer(answerId, adminAuth)
    })

    test('return 200 when updating points', async () => {
      payload = { points: 2 }

      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyAnswer(answerId, adminAuth)
    })

    test('return 200 when updating points', async () => {
      payload = { is_correct: false }

      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyAnswer(answerId, adminAuth)
    })

    test('return 200 when updating all answer properties', async () => {
      payload = {
        text: 'New Title',
        points: 2,
        is_correct: false,
      }

      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyAnswer(answerId, adminAuth)
    })
  })

  describe('when requestor is User', () => {
    test('return 200 when updating all answer properties', async () => {
      payload = {
        text: 'New Title',
        points: 2,
        is_correct: false,
      }

      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyAnswer(answerId, userAuth)
    })

    test('updates answer text', async () => {
      payload = {
        text: 'New Text',
      }
      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyAnswer(answerId, userAuth)
    })

    test('return 400 when text is invalid', async () => {
      payload = {
        text: '1',
      }
      await api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 403 when requestor does not own answer', async () => {
      const otherUserAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')

      return api
        .patch(testEndpoint(quizId, questionId, answerId))
        .set('Authorization', otherUserAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 404 code when answer does not exist', () => {
    answerId = 'cbd7a9ff-18f5-489e-b61e-cdd4a1394968'

    return api
      .patch(testEndpoint(quizId, questionId, answerId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })

  test('returns 404 code when answer does not belong to quiz', () => {
    quizId = '7070ba54-6ed7-4916-b3b6-e7251770d0b1'

    return api
      .patch(testEndpoint(quizId, questionId, answerId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: 'Question not found in quiz',
        status: 404,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.patch(testEndpoint(quizId, questionId, answerId)).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
