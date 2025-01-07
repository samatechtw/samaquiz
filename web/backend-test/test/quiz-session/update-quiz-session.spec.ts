import {
  ICreateQuizSessionApiResponse,
  IGetQuizSessionApiResponse,
  IUpdateQuizSessionApiRequest,
  QuizSessionStatus,
} from '@frontend/types'
import {
  testagent,
  TestAgent,
  adminAuthHeader,
  userAuthHeader,
  AppDbResetService,
} from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Update QuizSession', () => {
  const testEndpoint = (sessionId: string) => `/api/quiz_sessions/${sessionId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let sessionId: string
  let payload: IUpdateQuizSessionApiRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    sessionId = 'ce782f8a-01bb-4228-9e50-4db0248f14cd'
    payload = { code: 'newcode' }
  })

  const verifyQuizSession = async (auth: string, code?: string) => {
    const c = payload.code ?? code ?? 'abc'
    const response = await api
      .get(`/api/quiz_sessions/code/${c}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuizSessionApiResponse = response.body
    if (payload.code !== undefined) {
      expect(body.code).toEqual(payload.code.toLocaleLowerCase())
    }
    if (payload.host_name !== undefined) {
      expect(body.host_name).toEqual(payload.host_name)
    }
    if (payload.host_avatar !== undefined) {
      expect(body.host_avatar).toEqual(payload.host_avatar)
    }
    if (payload.start_time !== undefined) {
      expect(body.start_time).toEqual(payload.start_time)
    }
    if (payload.end_time !== undefined) {
      expect(body.end_time).toEqual(payload.end_time)
    }
    if (payload.question_end_time !== undefined) {
      expect(body.question_end_time).toEqual(payload.question_end_time)
    }
    if (payload.question_index !== undefined) {
      expect(body.question_index).toEqual(payload.question_index)
    }
    if (payload.question_duration !== undefined) {
      expect(body.question_duration).toEqual(payload.question_duration)
    }
    if (payload.status !== undefined) {
      expect(body.status).toEqual(payload.status)
    }
  }

  describe('when requestor is Admin', () => {
    test('return 200 when updating session text', async () => {
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(adminAuth)
    })

    test('return 200 when updating code and host_name', async () => {
      payload = {
        code: 'anotherCode',
        host_name: 'NEW NAME',
      }

      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(adminAuth)
    })

    test('return 200 when updating all properties', async () => {
      payload = {
        code: 'anotherCode',
        host_name: 'NEW NAME',
        host_avatar: 'http://localhost:8080/src/theme/img/cats/cat8.png',
        start_time: Date.now() + 60 * 60 * 1000,
      }

      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(adminAuth)
    })
  })

  describe('when requestor is User', () => {
    test('return 200 when updating all properties except status', async () => {
      payload = {
        code: 'anotherCode',
        host_name: 'NEW NAME',
        host_avatar: 'http://localhost:8080/src/theme/img/cats/cat8.png',
        start_time: Date.now() + 60 * 60 * 1000,
        question_duration: 45 * 1000,
      }

      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(userAuth)
    })

    test('updates status from Ready to Active', async () => {
      payload = {
        status: QuizSessionStatus.Active,
      }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(userAuth)
    })

    test('updates status from Active to Canceled', async () => {
      sessionId = 'ffb8d80d-086f-4ce3-999b-ae9842afb564'
      payload = {
        status: QuizSessionStatus.Canceled,
      }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(userAuth, 'activequiz')
    })

    test('updates status from Active to Complete with end_time', async () => {
      sessionId = 'ffb8d80d-086f-4ce3-999b-ae9842afb564'
      payload = {
        status: QuizSessionStatus.Complete,
        end_time: Date.now(),
      }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(userAuth, 'activequiz')
    })

    test('updates question_end_time and question_index', async () => {
      sessionId = 'ffb8d80d-086f-4ce3-999b-ae9842afb564'
      payload = {
        question_end_time: Date.now() + 35 * 1000,
        question_index: 1,
      }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(200)

      await verifyQuizSession(userAuth, 'activequiz')
    })

    test('return 400 when status updated incorrectly', async () => {
      // Active to Ready
      sessionId = 'ffb8d80d-086f-4ce3-999b-ae9842afb564'
      payload = { status: QuizSessionStatus.Ready }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidStatus',
          message: 'Invalid status update',
          status: 400,
        })
      // Canceled to Ready
      sessionId = '29ac8d30-9489-495d-8d79-677c04a4a9b8'
      payload = { status: QuizSessionStatus.Ready }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidStatus',
          message: 'Invalid status update',
          status: 400,
        })
      // Canceled to Active
      sessionId = '29ac8d30-9489-495d-8d79-677c04a4a9b8'
      payload = { status: QuizSessionStatus.Active }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidStatus',
          message: 'Invalid status update',
          status: 400,
        })
      // Canceled to Complete
      sessionId = '29ac8d30-9489-495d-8d79-677c04a4a9b8'
      payload = { status: QuizSessionStatus.Complete }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidStatus',
          message: 'Invalid status update',
          status: 400,
        })
      // Complete to Ready
      sessionId = 'd12520fe-7f5a-46a7-9682-f138c7f81078'
      payload = { status: QuizSessionStatus.Ready }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidStatus',
          message: 'Invalid status update',
          status: 400,
        })
      // Complete to Active
      sessionId = 'd12520fe-7f5a-46a7-9682-f138c7f81078'
      payload = { status: QuizSessionStatus.Active }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidStatus',
          message: 'Invalid status update',
          status: 400,
        })
    })

    test('return 400 when starting quiz with no questions', async () => {
      // Create quiz session with no questions
      userAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')
      const quizId = '7070ba54-6ed7-4916-b3b6-e7251770d0b1'
      payload = {
        code: 'my-quiz-code',
        host_name: 'My Name',
        host_avatar: 'http://localhost:8080/src/theme/img/cats/cat7.png',
      }
      const response = await api
        .post(`/api/quizzes/${quizId}/sessions`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(201)
      const body: ICreateQuizSessionApiResponse = response.body

      payload = { status: QuizSessionStatus.Active }
      await api
        .patch(testEndpoint(body.id))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'NoQuestions',
          message: 'Quiz must have questions',
          status: 400,
        })
    })

    test('return 400 when code is invalid', async () => {
      payload = {
        code: '',
      }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 400 when host_name is invalid', async () => {
      payload = { host_name: 'alsdkjflakjflakjsdlfkjasldkfjlakjflakjdsflajdasdf' }

      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })

      payload = { host_name: '1' }
      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 400 when host_avatar is invalid', async () => {
      payload = {
        host_avatar: '1234567890'.repeat(11),
      }

      await api
        .patch(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('return 403 when requestor does not own session', async () => {
      const otherUserAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')

      return api
        .patch(testEndpoint(sessionId))
        .set('Authorization', otherUserAuth)
        .send(payload)
        .expect({
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('returns 404 code when session does not exist', () => {
    sessionId = 'cbd7a9ff-18f5-489e-b61e-cdd4a1394968'

    return api
      .patch(testEndpoint(sessionId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect({
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })

  test('returns 401 when user is not authorized', async () => {
    await api.patch(testEndpoint(sessionId)).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
