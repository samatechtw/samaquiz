import {
  ICreateAnswerApiRequest,
  ICreateAnswerApiResponse,
  IGetQuestionApiResponse,
} from '@frontend/types'
import { commonRegex } from '@frontend/util/format'
import { AppDbResetService } from '@test/shared'
import { adminAuthHeader, testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Create Answer', () => {
  const testEndpoint = (questionId: string) => `/api/questions/${questionId}/answers`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateAnswerApiRequest
  let adminAuth: string
  let userAuth: string
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
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
    questionId = '813a13c9-4562-4fa3-8d23-f46a079a57de'
    payload = {
      text: 'My New Answer',
      is_correct: false,
      points: 1,
    }
  })

  const verifyAnswer = async (id: string, auth: string) => {
    const response = await api
      .get(`/api/questions/${questionId}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuestionApiResponse = response.body
    const answer = body.answers.find((a) => a.id === id)
    expect(answer).toBeDefined()
    expect(answer?.text).toEqual(payload.text)
    expect(answer?.is_correct).toEqual(payload.is_correct)
    expect(answer?.points).toEqual(payload.points)
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

  test('admin creates answer', async () => {
    const response = await api
      .post(testEndpoint(questionId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateAnswerApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyAnswer(body.id, adminAuth)

    // Verify id added to answers_order
    const response2 = await api
      .get(`/api/questions/${questionId}`)
      .set('Authorization', adminAuth)
      .expect(200)

    const question: IGetQuestionApiResponse = response2.body
    expect(question.answers.length).toEqual(3)
    expect(question.answers_order.length).toEqual(3)
    expect(question.answers_order[2]).toEqual(body.id)
  })

  test('user creates answer', async () => {
    const response = await api
      .post(testEndpoint(questionId))
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateAnswerApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyAnswer(body.id, userAuth)
  })

  describe('when request is not valid', () => {
    test('returns 400 code when text length is invalid', async () => {
      // Text too short
      payload.text = ''
      await api
        .post(testEndpoint(questionId))
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
        'a'.repeat(200)
      return api
        .post(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('returns 401 when user is not authorized', async () => {
      await api.post(testEndpoint(questionId)).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
