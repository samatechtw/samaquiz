import {
  IGetQuestionApiResponse,
  IUpdateQuestionApiRequest,
  QuestionType,
} from '@frontend/types'
import { testagent, TestAgent, adminAuthHeader, userAuthHeader } from '../helpers'
import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Update Question', () => {
  const testEndpoint = (questionId: string) => `/api/questions/${questionId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let questionId: string
  let payload: IUpdateQuestionApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    questionId = '11354d45-903d-4493-9b96-5f07497b01e1'
    payload = { text: 'Hello Question!' }
  })

  const verifyQuestion = async (id: string, auth: string) => {
    const response = await api
      .get(testEndpoint(questionId))
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuestionApiResponse = response.body
    if (payload.text !== undefined) {
      expect(body.text).toEqual(payload.text)
    }
    if (payload.answers_order !== undefined) {
      expect(body.answers_order).toEqual(payload.answers_order)
    }
    if (payload.asset_url !== undefined) {
      expect(body.asset_url).toEqual(payload.asset_url)
    }
    expect(body.question_type).toEqual(QuestionType.MultipleChoice)
  }

  describe('when requestor is Admin', () => {
    test('return 200 when updating question text', async () => {
      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuestion(questionId, adminAuth)
    })

    test('return 200 when updating answers_order', async () => {
      payload = {
        answers_order: [
          'a314a297-5d3c-42f3-a7a7-7d8726aafd1f',
          'f424b181-ec35-4a3b-9b81-d467cd0cbe7b',
          '66f41b1d-7777-4045-8ecf-30519a3a0a30',
          '869563bf-63b0-47d4-812e-be31e73fe79a',
        ],
      }

      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuestion(questionId, adminAuth)
    })

    test('return 200 when updating asset_url', async () => {
      payload = {
        asset_url: 'https://myassets.com/image.jpg',
      }

      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuestion(questionId, adminAuth)
    })

    test('return 200 when updating all question properties', async () => {
      payload = {
        text: 'New Title',
        answers_order: [
          'a314a297-5d3c-42f3-a7a7-7d8726aafd1f',
          'f424b181-ec35-4a3b-9b81-d467cd0cbe7b',
          '66f41b1d-7777-4045-8ecf-30519a3a0a30',
          '869563bf-63b0-47d4-812e-be31e73fe79a',
        ],
        asset_url: 'https://myassets.com/image.jpg',
      }

      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuestion(questionId, adminAuth)
    })
  })

  describe('when requestor is User', () => {
    test('return 200 when updating all question properties', async () => {
      payload = {
        text: 'New Title',
        answers_order: [
          'a314a297-5d3c-42f3-a7a7-7d8726aafd1f',
          'f424b181-ec35-4a3b-9b81-d467cd0cbe7b',
          '66f41b1d-7777-4045-8ecf-30519a3a0a30',
          '869563bf-63b0-47d4-812e-be31e73fe79a',
        ],
      }

      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuestion(questionId, userAuth)
    })

    test('updates question answers_order', async () => {
      payload = {
        answers_order: [
          'a314a297-5d3c-42f3-a7a7-7d8726aafd1f',
          'f424b181-ec35-4a3b-9b81-d467cd0cbe7b',
          '66f41b1d-7777-4045-8ecf-30519a3a0a30',
          '869563bf-63b0-47d4-812e-be31e73fe79a',
        ],
      }
      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuestion(questionId, userAuth)
    })

    test('return 400 when text is invalid', async () => {
      payload = {
        text: '1',
      }
      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 400 when answers_order is incorrect', async () => {
      payload = {
        answers_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
        ],
      }

      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'OrderLength',
          message: 'Failed to validate request',
          status: 400,
        })

      payload = {
        answers_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }
      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'OrderLength',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 400 when answers_order is invalid', async () => {
      payload = {
        answers_order: [
          'abcdefgi-7c60-4e39-b0e2-58bd8f6aa1cd',
          '12345678-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }

      await api
        .patch(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'OrderValue',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 403 when requestor does not own question', async () => {
      const otherUserAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')

      return api
        .patch(testEndpoint(questionId))
        .set('Authorization', otherUserAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 404 code when question does not exist', () => {
    questionId = 'cbd7a9ff-18f5-489e-b61e-cdd4a1394968'

    return api
      .patch(testEndpoint(questionId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.patch(testEndpoint(questionId)).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
