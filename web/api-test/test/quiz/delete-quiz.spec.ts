import { adminAuthHeader, TestAgent, testagent, userAuthHeader } from '../helpers'
import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, it } from 'vitest'

describe('Delete Quiz', () => {
  const testEndpoint = '/api/quizzes'
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let quizId: string

  beforeAll(() => {
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    adminAuth = adminAuthHeader()
    userAuth = userAuthHeader('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e')
    quizId = 'd6599ea6-818c-4687-8522-86bf880019c4'
  })

  describe('when requester is Admin', () => {
    it('returns 200 status code and message', async () => {
      await api
        .delete(`${testEndpoint}/${quizId}`)
        .set('Authorization', adminAuth)
        .expect(200)
    })

    it('returns 200 status code and message when delete quiz', async () => {
      await api
        .delete(`${testEndpoint}/${quizId}`)
        .set('Authorization', adminAuth)
        .expect(200)
    })
  })

  describe('when requester owns asset', () => {
    it('returns 200 status code and message', async () => {
      await api
        .delete(`${testEndpoint}/${quizId}`)
        .set('Authorization', userAuth)
        .expect(200)
    })

    it('returns 403 status when requester is not quiz owner', async () => {
      const newAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211') // user

      return api
        .delete(`${testEndpoint}/${quizId}`)
        .set('Authorization', newAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  describe('when request is invalid', () => {
    it('when quiz id does not exist', () => {
      const quizId = '1c2e5d05-7fa3-416d-985b-4cb9ee3ca6c5'

      return api
        .delete(`${testEndpoint}/${quizId}`)
        .set('Authorization', userAuth)
        .expect(404, {
          code: 'None',
          message: 'Not found',
          status: 404,
        })
    })

    it('when user is not authorized', async () => {
      return api.delete(`${testEndpoint}/${quizId}`).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
