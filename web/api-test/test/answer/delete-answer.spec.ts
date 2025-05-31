import { IGetQuestionApiResponse } from '@frontend/types'
import { adminAuthHeader, TestAgent, testagent, userAuthHeader } from '../helpers'
import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, expect, it } from 'vitest'

describe('Delete Answer', () => {
  const testEndpoint = (answerId: string) => `/api/answers/${answerId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let quizId: string
  let questionId: string
  let answerId: string

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
    questionId = '813a13c9-4562-4fa3-8d23-f46a079a57de'
    answerId = 'd06315f6-6304-45c0-9020-983b232e4701'
  })

  describe('when requester is Admin', () => {
    it('returns 200 status code and message', async () => {
      await api.delete(testEndpoint(answerId)).set('Authorization', adminAuth).expect(200)

      // Verify id removed from answers_order
      const quizResponse = await api
        .get(`/api/questions/${questionId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const quiz: IGetQuestionApiResponse = quizResponse.body
      expect(quiz.answers.length).toEqual(1)
      expect(quiz.answers_order.length).toEqual(1)
      const answer = quiz.answers.find((a) => a.id === answerId)
      expect(answer).toBeUndefined()
    })

    it('returns 200 status code and message when delete question', async () => {
      await api.delete(testEndpoint(answerId)).set('Authorization', adminAuth).expect(200)
    })
  })

  describe('when requester owns answer', () => {
    it('returns 200 status code and message', async () => {
      await api.delete(testEndpoint(answerId)).set('Authorization', userAuth).expect(200)
    })

    it('returns 403 status when requester is not answer owner', async () => {
      const newAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211') // user

      return api
        .delete(testEndpoint(answerId))
        .set('Authorization', newAuth)
        .expect(403, {
          code: 'None',
          message: 'Forbidden',
          status: 403,
        })
    })
  })

  describe('when request is invalid', () => {
    it('when user is not authorized', async () => {
      return api.delete(testEndpoint(answerId)).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
