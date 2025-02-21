import {
  ICreateQuizResponseApiRequest,
  ICreateQuizResponseApiResponse,
  IUpdateQuizSessionApiRequest,
} from '@frontend/types'
import { commonRegex } from '@frontend/util/format'
import {
  adminAuthHeader,
  AppDbResetService,
  testagent,
  TestAgent,
  userAuthHeader,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Create Quiz Response', () => {
  const testEndpoint = () => `/api/quiz_responses`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateQuizResponseApiRequest
  let adminAuth: string
  let userAuth: string
  let sessionId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    // quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
    sessionId = 'ffb8d80d-086f-4ce3-999b-ae9842afb564'
    payload = {
      participant_id: 'b4ca8f1d-f737-4e7d-9e92-350ae24cfa53',
      question_id: '11354d45-903d-4493-9b96-5f07497b01e1',
      answer_id: 'f424b181-ec35-4a3b-9b81-d467cd0cbe7b',
    }
  })

  test('admin creates correct quiz response', async () => {
    const response = await api
      .post(testEndpoint())
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizResponseApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    expect(body.is_correct).toBe(true)
  })

  test('user creates correct quiz response', async () => {
    const response = await api
      .post(testEndpoint())
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizResponseApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
  })

  test('anonymous creates correct quiz response', async () => {
    const response = await api
      .post(testEndpoint())
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizResponseApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    expect(body.is_correct).toBe(true)
  })

  test('anonymous creates correct quiz response', async () => {
    const response = await api
      .post(testEndpoint())
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizResponseApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    expect(body.is_correct).toBe(true)
  })

  test('anonymous creates incorrect quiz response', async () => {
    payload.answer_id = '66f41b1d-7777-4045-8ecf-30519a3a0a30'
    const response = await api
      .post(testEndpoint())
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizResponseApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    expect(body.is_correct).toBe(false)
  })

  describe('when request is not valid', () => {
    test('returns 400 when participant_id is not in session', async () => {
      payload.participant_id = '7c885923-a458-48f5-b48e-d84c7e995930'
      await api.post(testEndpoint()).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidQuestion', // Question doesn't match since it's a different session
      })
    })

    test('returns 400 when question is not active', async () => {
      payload.question_id = 'a15ff0f0-8825-4b64-8a16-2add5747bf42'
      await api.post(testEndpoint()).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidQuestion',
      })
    })

    test('returns 400 when question_end_time has passed', async () => {
      sessionId = 'ffb8d80d-086f-4ce3-999b-ae9842afb564'
      const updatePayload: IUpdateQuizSessionApiRequest = {
        question_end_time: Date.now() - 1000,
      }
      await api
        .patch(`/api/quiz_sessions/${sessionId}`)
        .set('Authorization', userAuth)
        .send(updatePayload)
        .expect(200)

      await api.post(testEndpoint()).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'QuestionOver',
      })
    })

    test('returns 400 when answer does not match question', async () => {
      payload.answer_id = 'd06315f6-6304-45c0-9020-983b232e4701'
      await api.post(testEndpoint()).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidAnswer',
      })
    })
  })
})
