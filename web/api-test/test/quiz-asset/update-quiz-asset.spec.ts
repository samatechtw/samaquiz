import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { adminAuthHeader, TestAgent, testagent, userAuthHeader } from '../helpers'
import { beforeAll, beforeEach, describe, expect, test } from 'vitest'
import {
  IUpdateQuizAssetRequest,
  IUpdateQuizAssetResponse,
  QuizAssetState,
} from '@frontend/types'

describe('Update Quiz Asset', () => {
  const testEndpoint = '/api/quiz_assets'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let userAssetId: string
  let payload: IUpdateQuizAssetRequest

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')

    userAssetId = '77bd4ae2-418c-43ef-8c16-ca5462bca1c3'

    payload = { state: QuizAssetState.Expired }
  })

  describe('when requester is Admin', () => {
    test('returns 200 status code and message when update state', async () => {
      const response = await api
        .patch(`${testEndpoint}/${userAssetId}`)
        .set('Authorization', adminAuth)
        .send(payload)
        .expect(200)

      const body: IUpdateQuizAssetResponse = response.body

      expect(body.state).toEqual(payload.state)
    })
  })

  describe('when requester is Owner', () => {
    test('returns 400 status when update state', async () => {
      return api
        .patch(`${testEndpoint}/${userAssetId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'None',
          message: 'Unauthorized field update',
          status: 400,
        })
    })
  })

  describe('when request is invalid', () => {
    test('when user requester is not asset owner', () => {
      const newAuth = userAuthHeader('749a72a8-14cd-4893-98eb-45bed891463d')

      return api
        .patch(`${testEndpoint}/${userAssetId}`)
        .set('Authorization', newAuth)
        .send(payload)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    test('when asset asset id does not exist', () => {
      const assetAssetId = '1c2e5d05-7fa3-416d-985b-4cb9ee3ca6c5'

      return api
        .patch(`${testEndpoint}/${assetAssetId}`)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(404, {
          code: 'None',
          message: 'Quiz asset not found',
          status: 404,
        })
    })

    test('when user is not authorized', () => {
      return api.patch(`${testEndpoint}/${userAssetId}`).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
