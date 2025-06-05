import { beforeAll, beforeEach, describe, it } from 'vitest'
import { adminAuthHeader, testagent, TestAgent, userAuthHeader } from '../helpers'
import { AppDbResetService } from '@test/shared'
import fs from 'fs'
import {
  AssetContentType,
  ICreateQuizAssetRequest,
  ICreateQuizAssetResponse,
} from '@frontend/types'
import { testConfig } from '../test.config'

describe('Delete Quiz Asset', { timeout: 10_000 }, () => {
  const testEndpoint = '/api/quiz_assets'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let adminPayload: ICreateQuizAssetRequest
  let ownerPayload: ICreateQuizAssetRequest
  let fileBuffer: Buffer

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
    fileBuffer = fs.readFileSync('./web/api-test/test/assets/test-asset.jpg')
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')

    adminPayload = {
      content_size: fileBuffer.length,
      content_type: AssetContentType.Jpeg,
      quiz_id: 'd6599ea6-818c-4687-8522-86bf880019c4',
    }
    ownerPayload = {
      content_size: fileBuffer.length,
      content_type: AssetContentType.Jpeg,
      quiz_id: 'efd3863f-a975-4b2d-9b03-eb3fe28410b9',
    }
  })

  const createAsset = async (
    payload: ICreateQuizAssetRequest,
    auth: string,
  ): Promise<ICreateQuizAssetResponse> => {
    const resOwner = await api
      .post(testEndpoint)
      .set('Authorization', auth)
      .send(payload)
      .expect(201)

    return resOwner.body
  }

  describe('when requester is Admin', () => {
    it('returns 200 status code and message', async () => {
      const adminAsset = await createAsset(adminPayload, adminAuth)
      // Upload asset
      await testagent(adminAsset.signed_url).put('').send(fileBuffer).expect(200)

      await api
        .delete(`${testEndpoint}/${adminAsset.id}`)
        .set('Authorization', adminAuth)
        .expect(200)
    })

    it('returns 200 status code and message when delete owner asset', async () => {
      const userAsset = await createAsset(ownerPayload, userAuth)
      // Upload asset
      await testagent(userAsset.signed_url).put('').send(fileBuffer).expect(200)

      await api
        .delete(`${testEndpoint}/${userAsset.id}`)
        .set('Authorization', adminAuth)
        .expect(200)
    })
  })

  describe('when requester is Owner', () => {
    it('returns 200 status code and message', async () => {
      const userAsset = await createAsset(ownerPayload, userAuth)
      // Upload asset
      await testagent(userAsset.signed_url).put('').send(fileBuffer).expect(200)

      await api
        .delete(`${testEndpoint}/${userAsset.id}`)
        .set('Authorization', userAuth)
        .expect(200)
    })

    it('returns 403 status when delete admin asset', async () => {
      const adminAsset = await createAsset(adminPayload, adminAuth)
      return api
        .delete(`${testEndpoint}/${adminAsset.id}`)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    it('returns 403 status when requester is not quiz owner', async () => {
      const userAsset = await createAsset(ownerPayload, userAuth)
      const newAuth = userAuthHeader('749a72a8-14cd-4893-98eb-45bed891463d') // user2@samatech.tw

      return api
        .delete(`${testEndpoint}/${userAsset.id}`)
        .set('Authorization', newAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  describe('when request is invalid', () => {
    it('when quiz asset id does not exist', () => {
      const quizAssetId = '1c2e5d05-7fa3-416d-985b-4cb9ee3ca6c5'

      return api
        .delete(`${testEndpoint}/${quizAssetId}`)
        .set('Authorization', userAuth)
        .expect(404, {
          code: 'None',
          message: 'Quiz asset not found',
          status: 404,
        })
    })

    it('when user is not authorized', async () => {
      const adminAsset = await createAsset(adminPayload, adminAuth)
      return api.delete(`${testEndpoint}/${adminAsset.id}`).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
