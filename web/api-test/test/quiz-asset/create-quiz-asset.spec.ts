import { beforeAll, expect, test, beforeEach, describe } from 'vitest'
import { AppDbResetService } from '@test/shared'
import { testagent, TestAgent, userAuthHeader } from '../helpers'
import { testConfig } from '../test.config'
import {
  AssetContentType,
  ICreateQuizAssetRequest,
  ICreateQuizAssetResponse,
} from '@frontend/types'
import { commonRegex } from '@frontend/util/format'

describe('Create Quiz Asset', { timeout: 10_000 }, () => {
  const testEndpoint = '/api/quiz_assets'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let payload: ICreateQuizAssetRequest
  let userAuth: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    payload = {
      content_size: 100000,
      content_type: AssetContentType.Jpeg,
      quiz_id: 'd6599ea6-818c-4687-8522-86bf880019c4',
    }
  })

  test('returns 201 status code and signed url', async () => {
    const response = await api
      .post(testEndpoint)
      .set('Authorization', userAuth)
      .send(payload)
      .expect(201)

    const body: ICreateQuizAssetResponse = response.body

    expect(body.id).toMatch(new RegExp(commonRegex.uuid))
    const uploadUrl = new URL(body.signed_url)
    expect(Object.keys(body)).toHaveLength(10)
    expect(body.quiz_id).toEqual(payload.quiz_id)
    expect(`${uploadUrl.protocol}//${uploadUrl.host}`).toMatch(commonRegex.assetUrl)
    expect(uploadUrl.pathname).toEqual(`/quiz-assets/${body.user_id}/${body.id}.jpg`)
    expect(uploadUrl.searchParams.get('X-Amz-Expires')).toEqual('600')
    expect(uploadUrl.searchParams.get('X-Amz-Signature')?.length).toBeGreaterThan(20)
  })

  describe('when request is invalid', () => {
    test('when owner requester is not quiz owner', () => {
      payload.quiz_id = '7070ba54-6ed7-4916-b3b6-e7251770d0b1'
      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(403, {
          code: 'None',
          message: 'User does not own quiz',
          status: 403,
        })
    })

    test('when no quiz id is provided', () => {
      const { quiz_id: _quiz_id, ...badPayload } = payload
      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(badPayload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'missing field `quiz_id`',
          status: 400,
        })
    })

    test('when user is not authorized', () => {
      return api.post(testEndpoint).send(payload).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })

    test('when content_size is not valid', async () => {
      payload.content_size = 1000000001

      await api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })

      payload.content_size = -1
      await api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message: 'Failed to validate request',
          status: 400,
        })
    })

    test('when asset usage is exceeded', async () => {
      payload.content_size = 20_000_000

      await api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'AssetUsageExceeded',
          message: 'Asset usage exceeded limit',
          status: 400,
        })
    })

    test('when content_type is not valid', () => {
      payload.content_type = 'text/plain' as AssetContentType

      return api
        .post(testEndpoint)
        .set('Authorization', userAuth)
        .send(payload)
        .expect(400, {
          code: 'InvalidFormData',
          message:
            'unknown variant `text/plain`, expected one of `image/jpeg`, `image/png`, `image/webp`, `image/svg+xml`, `image/gif`, `video/mp4`, `application/pdf`',
          status: 400,
        })
    })
  })
})
