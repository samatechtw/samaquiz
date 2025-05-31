import {
  ICreateQuizApiRequest,
  ICreateQuizApiResponse,
  IGetQuizApiResponse,
  QuizType,
} from '@frontend/types'
import { commonRegex } from '@frontend/util/format'
import { adminAuthHeader, testagent, TestAgent, userAuthHeader } from '../helpers'
import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Create Quiz', () => {
  const testEndpoint = '/api/quizzes'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateQuizApiRequest
  let adminAuth: string
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    payload = {
      title: 'My New Quiz',
      description: 'Welcome to my new quiz, it is very nice',
    }
  })

  const verifyQuiz = async (id: string, auth: string) => {
    const response = await api
      .get(`${testEndpoint}/${id}`)
      .set('Authorization', auth)
      .expect(200)

    const body: IGetQuizApiResponse = response.body
    expect(body.title).toEqual(payload.title)
    expect(body.description).toEqual(payload.description)
    expect(body.quiz_type).toEqual(QuizType.Quiz)
    expect(body.questions_order).toEqual([])
    expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
  }

  test('admin creates quiz', async () => {
    const response = await api
      .post(testEndpoint)
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuiz(body.id, adminAuth)
  })

  test('user creates quiz', async () => {
    const response = await api
      .post(testEndpoint)
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateQuizApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyQuiz(body.id, userAuth)
  })

  describe('when request is not valid', () => {
    test('returns 400 code when name length is invalid', async () => {
      // Name too short
      payload.title = 'a'
      await api.post(testEndpoint).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
      })

      // Name too long
      payload.title =
        '12345678901234567890123456789012345678901234567890zabcdefghijklm' +
        'a'.repeat(80)
      return api.post(testEndpoint).set('Authorization', userAuth).send(payload).expect({
        status: 400,
        message: 'Failed to validate request',
        code: 'InvalidFormData',
      })
    })

    test('when description length is invalid', () => {
      payload.description = '1234'

      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('returns 401 when user is not authorized', async () => {
      await api.post(testEndpoint).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
