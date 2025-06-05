import fs from 'fs'
import {
  AssetContentType,
  ICreateQuizAssetRequest,
  ICreateQuizAssetResponse,
  IListQuizAssetsRequest,
  IListQuizAssetsResponse,
  IVerifyQuizAssetResponse,
  QuizAssetState,
  SortDirection,
} from '@frontend/types'
import { AppDbResetService } from '@test/shared'
import { beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { testConfig } from '../test.config'
import { TestAgent, testagent, userAuthHeader } from '../helpers'

describe('Verify Quiz Asset', { timeout: 10_000 }, () => {
  const testEndpoint = '/api/quiz_assets'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let ownerId: string
  let userAuth: string
  let fileBuffer: Buffer

  let quizAsset: ICreateQuizAssetResponse

  function verifyEndpoint(assetId: string): string {
    return `${testEndpoint}/${assetId}/actions/verify`
  }

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    ownerId = '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e'
    userAuth = userAuthHeader(ownerId)

    fileBuffer = fs.readFileSync('./web/api-test/test/assets/test-asset.jpg')

    const payload: ICreateQuizAssetRequest = {
      content_size: fileBuffer.length,
      content_type: AssetContentType.Jpeg,
      quiz_id: 'd6599ea6-818c-4687-8522-86bf880019c4',
    }

    const response = await api
      .post(testEndpoint)
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)

    quizAsset = response.body
  })

  test('returns 200 status code and message', async () => {
    // Upload asset
    await testagent(quizAsset.signed_url).put('').send(fileBuffer).expect(200)

    const response = await api
      .post(verifyEndpoint(quizAsset.id))
      .set('Authorization', userAuth)
      .expect(200)

    const body: IVerifyQuizAssetResponse = response.body
    expect(body.verified).toEqual(true)

    // Check latest quiz asset
    const query: IListQuizAssetsRequest = {
      user_id: ownerId,
      column: 'created_at',
      direction: SortDirection.Desc,
    }

    const listResponse = await api
      .get(testEndpoint)
      .query(query)
      .set('Authorization', userAuth)
      .expect(200)
    const listBody: IListQuizAssetsResponse = listResponse.body
    const quizs = listBody.results

    expect(quizs[0].id).toEqual(quizAsset.id)
    expect(quizs[0].state).toEqual(QuizAssetState.Uploaded)
  })

  describe('when request is invalid', () => {
    test('when owner requester is not quiz owner', () => {
      const newAuth = userAuthHeader('749a72a8-14cd-4893-98eb-45bed891463d')

      return api
        .post(verifyEndpoint(quizAsset.id))
        .set('Authorization', newAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })

    test('when quiz asset id does not exist', () => {
      const quizAssetId = '1c2e5d05-7fa3-416d-985b-4cb9ee3ca6c5'

      return api
        .post(verifyEndpoint(quizAssetId))
        .set('Authorization', userAuth)
        .expect(404, {
          code: 'None',
          message: 'Quiz asset not found',
          status: 404,
        })
    })

    test('when user is not authorized', () => {
      return api.post(verifyEndpoint(quizAsset.id)).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
