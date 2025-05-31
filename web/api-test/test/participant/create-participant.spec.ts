import {
  ICreateParticipantApiRequest,
  ICreateParticipantApiResponse,
  IGetParticipantApiResponse,
} from '@frontend/types'
import { commonRegex } from '@frontend/util/format'
import { AppDbResetService } from '@test/shared'
import { adminAuthHeader, testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'

describe('Create Participant', () => {
  const testEndpoint = (sessionId: string) => `/api/quiz_sessions/${sessionId}/join`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateParticipantApiRequest
  let adminAuth: string
  let userId: string
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
    userId = '028ba9f2-f360-423b-83b6-44863b69e211'
    userAuth = userAuthHeader(userId)
    sessionId = 'dab9771d-e818-435f-98de-6a734189ba7d'
    payload = {
      name: 'My Name',
      avatar: 'http://localhost:8080/src/theme/img/cats/cat10.png',
    }
  })

  const verifyParticipant = async (id: string, userId?: string) => {
    const response = await api.get(`/api/participants/${id}`).expect(200)

    const body: IGetParticipantApiResponse = response.body
    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    expect(body.session_id).toEqual(sessionId)
    expect(body.user_id).toEqual(userId || null)
    expect(body.name).toEqual(payload.name)
    expect(body.avatar).toEqual(payload.avatar)
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
  }

  test('admin creates participant', async () => {
    const adminId = '083fb9af-a2fd-41b8-bfa2-4747cc87b604'

    const response = await api
      .post(testEndpoint(sessionId))
      .set('Authorization', adminAuth)
      .send(payload)
      .expect(201)
    const body: ICreateParticipantApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyParticipant(body.id, adminId)
  })

  test('user creates participant', async () => {
    const response = await api
      .post(testEndpoint(sessionId))
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)
    const body: ICreateParticipantApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyParticipant(body.id, userId)
  })

  test('creates participant when user is not authorized', async () => {
    const response = await api.post(testEndpoint(sessionId)).send(payload).expect(201)
    const body: ICreateParticipantApiResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    await verifyParticipant(body.id)
  })

  describe('when request is not valid', () => {
    test('returns 400 code when name length is invalid', async () => {
      // Name too short
      payload.name = 'a'
      await api
        .post(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })

      // Name too long
      payload.name = 'abcdefg1234567890'
      return api
        .post(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('returns 400 code when avatar length is invalid', async () => {
      // Avatar too long
      payload.avatar = '345678901lm'.repeat(10)
      return api
        .post(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Failed to validate request',
          code: 'InvalidFormData',
        })
    })

    test('returns 400 code when quiz session is complete', async () => {
      sessionId = 'd12520fe-7f5a-46a7-9682-f138c7f81078'
      return api
        .post(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Cannot create participant',
          code: 'QuizSessionComplete',
        })
    })

    test('returns 400 code when quiz session is canceled', async () => {
      sessionId = '29ac8d30-9489-495d-8d79-677c04a4a9b8'
      return api
        .post(testEndpoint(sessionId))
        .set('Authorization', userAuth)
        .send(payload)
        .expect({
          status: 400,
          message: 'Cannot create participant',
          code: 'QuizSessionCanceled',
        })
    })
  })
})
