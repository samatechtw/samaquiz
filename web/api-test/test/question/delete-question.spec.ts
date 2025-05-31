import { IGetQuizApiResponse } from '@frontend/types'
import { adminAuthHeader, TestAgent, testagent, userAuthHeader } from '../helpers'
import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import { beforeAll, beforeEach, describe, expect, it } from 'vitest'

describe('Delete Question', () => {
  const testEndpoint = (questionId: string) => `/api/questions/${questionId}`
  let api: TestAgent
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let adminAuth: string
  let userAuth: string
  let quizId: string
  let questionId: string

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
    questionId = '11354d45-903d-4493-9b96-5f07497b01e1'
  })

  describe('when requester is Admin', () => {
    it('returns 200 status code and message', async () => {
      await api
        .delete(testEndpoint(questionId))
        .set('Authorization', adminAuth)
        .expect(200)

      // Verify id removed from questions_order
      const quizResponse = await api
        .get(`/api/quizzes/${quizId}`)
        .set('Authorization', adminAuth)
        .expect(200)

      const quiz: IGetQuizApiResponse = quizResponse.body
      expect(quiz.questions.length).toEqual(3)
      expect(quiz.questions_order.length).toEqual(3)
      const question = quiz.questions.find((q) => q.id === questionId)
      expect(question).toBeUndefined()
    })

    it('returns 200 status code and message when delete question', async () => {
      await api
        .delete(testEndpoint(questionId))
        .set('Authorization', adminAuth)
        .expect(200)
    })
  })

  describe('when requester owns question', () => {
    it('returns 200 status code and message', async () => {
      await api
        .delete(testEndpoint(questionId))
        .set('Authorization', userAuth)
        .expect(200)
    })

    it('returns 403 status when requester is not question owner', async () => {
      const newAuth = userAuthHeader('028ba9f2-f360-423b-83b6-44863b69e211') // user

      return api
        .delete(testEndpoint(questionId))
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
      return api.delete(testEndpoint(questionId)).expect(401, {
        code: 'Unauthorized',
        message: 'Unauthorized',
        status: 401,
      })
    })
  })
})
