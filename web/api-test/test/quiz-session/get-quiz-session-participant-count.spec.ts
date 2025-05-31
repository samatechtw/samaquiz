import { IGetParticipantCountApiResponse } from '@frontend/types'
import { AppDbResetService } from '@test/shared'
import { testagent, TestAgent } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'

describe('Get Quiz Session Participant Response', () => {
  const testEndpoint = (sessionId: string) =>
    `/api/quiz_sessions/${sessionId}/queries/participant_count`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let sessionId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    sessionId = 'ce782f8a-01bb-4228-9e50-4db0248f14cd'
  })

  test('returns 200 and participant count when user is Admin', async () => {
    const response = await api
      .get(testEndpoint(sessionId))
      .set('Authorization', adminAuth)
      .expect(200)

    const body: IGetParticipantCountApiResponse = response.body
    expect(body.count).toEqual(3)
  })

  test('returns 200 and participant count when user is Owner', async () => {
    const response = await api
      .get(testEndpoint(sessionId))
      .set('Authorization', adminAuth)
      .expect(200)

    const body: IGetParticipantCountApiResponse = response.body
    expect(body.count).toEqual(3)
  })

  test('returns 403 when anonymous user gets a quiz session', async () => {
    await api.get(testEndpoint(sessionId)).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })

  test('returns 403 error when user gets another user participants', async () => {
    const otherUserAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211')
    await api
      .get(testEndpoint(sessionId))
      .set('Authorization', otherUserAuth)
      .expect(403, {
        code: 'None',
        message: 'Forbidden',
        status: 403,
      })
  })

  test('returns 404 when quiz session does not exist', async () => {
    const nonExistId = '870aafc9-36e9-476a-b38c-c1aaaad9d9fe'
    await api.get(testEndpoint(nonExistId)).set('Authorization', adminAuth).expect(404, {
      code: 'None',
      message: 'Not found',
      status: 404,
    })
  })
})
