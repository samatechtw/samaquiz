import { IGetParticipantApiResponse } from '@frontend/types'
import { testagent, TestAgent, AppDbResetService } from '../helpers'
import { describe, expect, test, beforeAll, beforeEach } from 'vitest'
import { adminAuthHeader, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import { commonRegex } from '@frontend/util/format'

describe('Get Participant', () => {
  const testEndpoint = (participantId: string) => `/api/participants/${participantId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let sessionId: string
  let participantId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    adminAuth = adminAuthHeader()
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    sessionId = 'ce782f8a-01bb-4228-9e50-4db0248f14cd'
    participantId = 'bf78d346-09b2-4512-b1fd-2413b6b68c5b'
  })

  describe('when requester is Admin', () => {
    test('returns 200 and participant with user_id', async () => {
      participantId = '9ee64573-c84f-439e-8cc7-a0b7b1f1906f'
      const response = await api
        .get(testEndpoint(participantId))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetParticipantApiResponse = response.body
      expect(body.id).toEqual(participantId)
      expect(body.session_id).toEqual(sessionId)
      expect(body.user_id).toBe('028ba9f2-f360-423b-83b6-44863b69e211')
      expect(body.name).toEqual('S1P1')
      expect(body.avatar).toEqual('http://localhost:8080/src/theme/img/cats/cat2.png')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
    })

    test('returns participant with no user_id', async () => {
      const response = await api
        .get(testEndpoint(participantId))
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IGetParticipantApiResponse = response.body
      expect(body.id).toEqual(participantId)
      expect(body.session_id).toEqual(sessionId)
      expect(body.user_id).toBe(null)
      expect(body.name).toEqual('S1P2')
      expect(body.avatar).toEqual('http://localhost:8080/src/theme/img/cats/cat3.png')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
    })
  })

  describe('when requester is User', () => {
    let userAuth: string

    beforeEach(() => {
      userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    })

    test('returns 200 and another participant', async () => {
      participantId = 'a7d74cb8-17a0-499d-b47d-0b8e1d1b3cdf'

      const response = await api
        .get(testEndpoint(participantId))
        .set('Authorization', userAuth)
        .expect(200)
      const body: IGetParticipantApiResponse = response.body

      expect(body.id).toEqual(participantId)
      expect(body.session_id).toEqual('dab9771d-e818-435f-98de-6a734189ba7d')
      expect(body.user_id).toBe(null)
      expect(body.name).toEqual('S2P1')
      expect(body.avatar).toEqual('http://localhost:8080/src/theme/img/cats/cat5.png')
      expect(body.created_at).toMatch(new RegExp(commonRegex.date))
    })
  })

  test('returns participant when requester is anonymous', async () => {
    participantId = 'ea1fcc27-a573-4a1b-9604-73591d089e14'

    const response = await api.get(testEndpoint(participantId)).expect(200)
    const body: IGetParticipantApiResponse = response.body

    expect(body.id).toEqual(participantId)
    expect(body.session_id).toEqual('29ac8d30-9489-495d-8d79-677c04a4a9b8')
    expect(body.user_id).toBe(null)
    expect(body.name).toEqual('S6P1')
    expect(body.avatar).toEqual('http://localhost:8080/src/theme/img/cats/cat8.png')
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
  })

  test('returns participant with relations when requester is anonymous', async () => {
    participantId = 'b4ca8f1d-f737-4e7d-9e92-350ae24cfa53'

    const response = await api.get(testEndpoint(participantId)).expect(200)
    const body: IGetParticipantApiResponse = response.body

    // TODO -- check response relations

    expect(body.id).toEqual(participantId)
    expect(body.session_id).toEqual('ffb8d80d-086f-4ce3-999b-ae9842afb564')
    expect(body.user_id).toBe(null)
    expect(body.name).toEqual('S3P1')
    expect(body.avatar).toEqual('http://localhost:8080/src/theme/img/cats/cat7.png')
    expect(body.created_at).toMatch(new RegExp(commonRegex.date))
  })

  test('returns 404 when participant does not exist', async () => {
    const nonExistId = '870aafc9-36e9-476a-b38c-c1aaaad9d9fe'
    await api.get(testEndpoint(nonExistId)).set('Authorization', adminAuth).expect(404, {
      code: 'None',
      message: 'Not found',
      status: 404,
    })
  })
})
