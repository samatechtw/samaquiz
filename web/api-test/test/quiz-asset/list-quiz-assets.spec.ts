import {
  adminAuthHeader,
  expiredAdminToken,
  expiredUser1Token,
  testagent,
  TestAgent,
  userAuthHeader,
} from '../helpers'
import { AppDbResetService } from '@test/shared'
import { beforeAll, beforeEach, describe, expect, test } from 'vitest'
import { testConfig } from '../test.config'
import {
  AssetContentType,
  IListQuizAssetsRequest,
  IListQuizAssetsResponse,
  QuizAssetState,
  SortDirection,
} from '@frontend/types'

const ASSET_ADMIN = 0
const ASSET_USER1 = 97040004
const ASSET_USER3 = 1000002

describe('List Quiz Assets', () => {
  const testEndpoint = '/api/quiz_assets'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
  })

  describe('when requester is Admin', () => {
    test('returns 200 status and assets when filtered by content_type', async () => {
      const query: IListQuizAssetsRequest = {
        content_type: AssetContentType.Jpeg,
      }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(94040002)
      const assets = body.results

      expect(assets.length).toEqual(6)
      expect(assets[0].id).toEqual('e79912c3-2647-48e1-96c3-5a76e86befbc')
      expect(assets[0].content_type).toEqual(AssetContentType.Jpeg)
      expect(assets[0].state).toEqual(QuizAssetState.Uploaded)
      expect(assets[0].user_id).toEqual('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
      expect(assets[0].quiz_id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')

      expect(assets[1].id).toEqual('77bd4ae2-418c-43ef-8c16-ca5462bca1c3')
      expect(assets[1].content_type).toEqual(AssetContentType.Jpeg)
      expect(assets[1].state).toEqual(QuizAssetState.Created)
      expect(assets[1].user_id).toEqual('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
      expect(assets[1].quiz_id).toEqual('d6599ea6-818c-4687-8522-86bf880019c4')
    })

    test('returns 200 status and assets when filtered by user_id', async () => {
      const query: IListQuizAssetsRequest = {
        user_id: '028ba9f2-f360-423b-83b6-44863b69e211',
      }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_USER3)
      const assets = body.results

      expect(assets.length).toEqual(1)
      expect(assets[0].id).toEqual('2bd4ed9b-7a7e-4f2c-b46c-24780ea53053')
    })

    test('returns 200 status and all assets when no query params are provided ', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_ADMIN + ASSET_USER1 + ASSET_USER3)
      const assets = body.results

      expect(assets.length).toEqual(9)
    })

    test('returns 200 status and empty response when querying for non-existent assets', async () => {
      const query: IListQuizAssetsRequest = {
        content_type: AssetContentType.Gif,
      }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(0)
      const assets = body.results

      expect(assets.length).toEqual(0)
    })

    test('returns 200 status and assets when sorted by created_at ASC', async () => {
      const query: IListQuizAssetsRequest = {
        column: 'created_at',
        direction: SortDirection.Asc,
      }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', adminAuth)
        .expect(200)

      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_ADMIN + ASSET_USER1 + ASSET_USER3)
      const assets = body.results

      expect(assets.length).toEqual(9)
      expect(assets[0].id).toEqual('73d117e0-e435-4b95-b42f-e81bbb0943ab')
      expect(assets[1].id).toEqual('2bd4ed9b-7a7e-4f2c-b46c-24780ea53053')
      expect(assets[2].id).toEqual('7d43646e-c438-4f26-9170-6a841a9df551')
      expect(assets[3].id).toEqual('1ee59f27-56bb-4081-a106-3f2fb70afd11')
      expect(assets[4].id).toEqual('51b9b547-8409-4017-926d-245e131edf70')
      expect(assets[5].id).toEqual('8041973b-be11-4eb2-8572-ec5e43008ef6')
      expect(assets[6].id).toEqual('8ce7d0d7-434a-4cb9-8edb-810412121d79')
      expect(assets[7].id).toEqual('77bd4ae2-418c-43ef-8c16-ca5462bca1c3')
      expect(assets[8].id).toEqual('e79912c3-2647-48e1-96c3-5a76e86befbc')
    })

    test('returns 400 status when user_id is not valid', async () => {
      const query: IListQuizAssetsRequest = {
        user_id: '12345',
      }
      await api
        .get(testEndpoint)
        .set('Authorization', adminAuth)
        .query(query)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('returns 401 when admin token has expired', async () => {
      await api.get(testEndpoint).set('Authorization', expiredAdminToken).expect(401, {
        code: 'InvalidAuth',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })

  describe('when requester is Owner', () => {
    test('returns 200 status and assets when filtering by user_id', async () => {
      const query: IListQuizAssetsRequest = {
        user_id: '2213d9fc-3693-47ed-a495-cd5e7fc6dd0e',
      }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)

      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_USER1)
      const assets = body.results

      expect(assets.length).toEqual(8)
    })

    test('returns 200 status with no filters', async () => {
      const response = await api
        .get(testEndpoint)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_USER1)
      const assets = body.results

      expect(assets.length).toEqual(8)
    })

    test('returns 200 status when filtering by content_type Png and state', async () => {
      const query: IListQuizAssetsRequest = {
        content_type: AssetContentType.Png,
        state: QuizAssetState.Uploaded,
      }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_USER1)
      const assets = body.results

      expect(assets.length).toEqual(1)
      expect(assets[0].id).toEqual('73d117e0-e435-4b95-b42f-e81bbb0943ab')
    })

    test('returns 200 status when filtering by content_type Svg', async () => {
      const query: IListQuizAssetsRequest = {
        content_type: AssetContentType.Svg,
      }
      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(0)
      const assets = body.results

      expect(assets.length).toEqual(0)
    })

    test('returns 200 status when filtering by quiz_id', async () => {
      const query: IListQuizAssetsRequest = {
        quiz_id: 'd6599ea6-818c-4687-8522-86bf880019c4',
      }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_USER1)
      const assets = body.results

      expect(assets.length).toEqual(4)
      expect(assets[0].id).toEqual('e79912c3-2647-48e1-96c3-5a76e86befbc')
      expect(assets[1].id).toEqual('77bd4ae2-418c-43ef-8c16-ca5462bca1c3')
      expect(assets[2].id).toEqual('8ce7d0d7-434a-4cb9-8edb-810412121d79')
      expect(assets[3].id).toEqual('51b9b547-8409-4017-926d-245e131edf70')
    })

    test('returns 200 status when ordering by size', async () => {
      const query: IListQuizAssetsRequest = {
        direction: SortDirection.Desc,
        column: 'size',
      }

      const response = await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(200)
      const body: IListQuizAssetsResponse = response.body
      expect(body.total_usage).toEqual(ASSET_USER1)
      const assets = body.results

      expect(assets.length).toEqual(8)
      expect(assets[0].id).toEqual('77bd4ae2-418c-43ef-8c16-ca5462bca1c3')
      expect(assets[1].id).toEqual('73d117e0-e435-4b95-b42f-e81bbb0943ab')
      expect(assets[2].id).toEqual('7d43646e-c438-4f26-9170-6a841a9df551')
    })

    test('returns 401 when owner token has expired', async () => {
      await api.get(testEndpoint).set('Authorization', expiredUser1Token).expect(401, {
        code: 'InvalidAuth',
        message: 'Unauthorized',
        status: 401,
      })
    })

    test('returns 403 status when filtering by other user_id', async () => {
      const query: IListQuizAssetsRequest = {
        // Owner's ID different from the requester's ID
        user_id: '028ba9f2-f360-423b-83b6-44863b69e211',
      }
      await api
        .get(testEndpoint)
        .query(query)
        .set('Authorization', userAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  test('When user is not authorized', async () => {
    await api.get(testEndpoint).expect(401, {
      code: 'Unauthorized',
      message: 'Unauthorized',
      status: 401,
    })
  })
})
