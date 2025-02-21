import { IUpdateQuizApiResponse, IUpdateQuizApiRequest, QuizType } from '@frontend/types'
import {
  testagent,
  TestAgent,
  adminAuthHeader,
  userAuthHeader,
  AppDbResetService,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Update Quiz', () => {
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let quizId: string
  let payload: IUpdateQuizApiRequest

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
  })

  describe('when requestor is Admin', () => {
    test('return 200 when updating quiz title', async () => {
      payload = { title: 'Hello Quiz!' }

      const response = await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateQuizApiResponse = response.body
      expect(body.title).toEqual(payload.title)
    })

    test('return 200 when updating questions_order', async () => {
      payload = {
        questions_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }

      const response = await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateQuizApiResponse = response.body
      expect(body.questions_order).toEqual(payload.questions_order)
    })

    test('return 200 when updating all quiz properties', async () => {
      payload = {
        title: 'New Title',
        description: 'New Description',
        questions_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }

      const response = await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateQuizApiResponse = response.body
      expect(body.title).toEqual(payload.title)
      expect(body.description).toEqual(payload.description)
      expect(body.questions_order).toEqual(payload.questions_order)
    })
  })

  describe('when requestor is User', () => {
    test('return 200 when updating all quiz properties', async () => {
      payload = {
        title: 'New Title',
        description: 'New Description',
        questions_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }

      const response = await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateQuizApiResponse = response.body
      expect(body.title).toEqual(payload.title)
      expect(body.description).toEqual(payload.description)
      expect(body.questions_order).toEqual(payload.questions_order)
    })

    test('updates quiz questions_order', async () => {
      payload = {
        questions_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }
      await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)
    })

    test('return 400 when title is invalid', async () => {
      payload = {
        title: '1',
      }
      await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 400 when questions_order is too short', async () => {
      payload = {
        questions_order: [
          '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd',
          '11354d45-903d-4493-9b96-5f07497b01e1',
        ],
      }

      await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'OrderLength',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 400 when questions_order is invalid', async () => {
      payload = {
        questions_order: [
          'abcdefgi-7c60-4e39-b0e2-58bd8f6aa1cd',
          '12345678-903d-4493-9b96-5f07497b01e1',
          '813a13c9-4562-4fa3-8d23-f46a079a57de',
          'a82c0a88-10eb-43cd-b057-7214bb598111',
        ],
      }

      await api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'OrderValue',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 403 when requestor does not own quiz', async () => {
      const otherUserAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')

      return api
        .patch(`/api/quizzes/${quizId}`)
        .set('Authorization', otherUserAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 404 code when quiz does not exist', () => {
    const id = 'cbd7a9ff-18f5-489e-b61e-cdd4a1394968'

    return api
      .patch(`/api/quizzes/${id}`)
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.patch(`/api/quizzes/${quizId}`).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
