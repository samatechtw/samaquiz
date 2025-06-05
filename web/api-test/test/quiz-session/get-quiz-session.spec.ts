import { IGetQuizSessionApiResponse, QuizSessionStatus } from '@frontend/types'
import { AppDbResetService } from '@test/shared'
import { testagent, TestAgent } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { commonRegex } from '@frontend/util/format'

describe('Get Quiz Session', () => {
  const testEndpoint = (code: string) => `/api/quiz_sessions/code/${code}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let quizId: string
  let userId: string
  let sessionId: string
  let code: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
    userId = '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e'
    sessionId = 'dab9771d-e818-435f-98de-6a734189ba7d'
    code = '123abc'
  })

  describe('when requester is Admin', () => {
    test('returns 200 and quiz session', async () => {
      const response = await api
        .get(testEndpoint(code))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuizSessionApiResponse = response.body
      expect(body.id).toEqual(sessionId)
      expect(body.user_id).toEqual(userId)
      expect(body.quiz.id).toEqual(quizId)
      expect(body.quiz.intro_background_url).toEqual(
        'https://img.freepik.com/free-photo/plain-smooth-green-wall-texture_53876-129746.jpg',
      )
      expect(body.code).toEqual(code)
      expect(body.host_name).toEqual('Host 2')
      expect(body.host_avatar).toEqual(
        'http://localhost:8080/src/theme/img/cats/cat3.png',
      )
      expect(body.start_time).toBeDefined()
      expect(body.end_time).toBeNull()
      expect(body.status).toEqual(QuizSessionStatus.Ready)
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader(userId)
    })

    test('returns 200 and session with relations when getting own quiz session', async () => {
      const response = await api
        .get(testEndpoint(code))
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetQuizSessionApiResponse = response.body

      expect(body.id).toEqual(sessionId)
      expect(body.user_id).toEqual(userId)
      expect(body.quiz.id).toEqual(quizId)
      expect(body.code).toEqual(code)
      expect(body.start_time).toBeDefined()
      expect(body.end_time).toBeNull()
      expect(body.status).toEqual(QuizSessionStatus.Ready)
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
      expect(body.updated_at).toMatch(new RegExp(commonRegex.date))

      expect(body.participants?.length).toEqual(2)
      expect(body.participants?.[0].id).toEqual('a7d74cb8-17a0-499d-b47d-0b8e1d1b3cdf')
      expect(body.participants?.[1].id).toEqual('b6a25521-bc3b-4b05-974d-6b120ac8b467')
    })

    test('returns quiz session when user gets another user quiz session', async () => {
      userAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')

      const response = await api
        .get(testEndpoint(code))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuizSessionApiResponse = response.body
      expect(body.code).toEqual(code)
    })

    test('returns quiz session with mixed case code', async () => {
      const mixed = '123aBc'

      const response = await api
        .get(testEndpoint(mixed))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuizSessionApiResponse = response.body
      expect(body.code).toEqual(code)
      expect(body.id).toEqual(sessionId)
    })

    test('returns quiz session with uppercase code', async () => {
      const upper = '123ABC'

      const response = await api
        .get(testEndpoint(upper))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetQuizSessionApiResponse = response.body
      expect(body.code).toEqual(code)
      expect(body.id).toEqual(sessionId)
    })
  })

  test('returns quiz session when anonymous user gets a quiz session', async () => {
    const response = await api.get(testEndpoint(code)).expect(200)

    const body: IGetQuizSessionApiResponse = response.body
    expect(body.code).toEqual(code)
  })

  test('returns 404 when quiz session does not exist', async () => {
    const nonExistCode = 'fake-coderoo'
    await api
      .get(testEndpoint(nonExistCode))
      .set('Authorization', adminAuth)
      .expect(404, {
        code: 'None',
        message: 'Not found',
        status: 404,
      })
  })
})
