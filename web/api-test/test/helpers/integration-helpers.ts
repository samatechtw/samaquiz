import {
  ICreateParticipantApiRequest,
  ICreateParticipantApiResponse,
  ICreateQuizResponseApiRequest,
  IUpdateQuizSessionApiRequest,
  QuizSessionStatus,
} from '@frontend/types'
import { commonRegex } from '@frontend/util/format'
import TestAgent from 'supertest/lib/agent'
import { expect } from 'vitest'

export const createParticipant = async (
  api: TestAgent,
  sessionId: string,
  payload: ICreateParticipantApiRequest,
) => {
  const response = await api
    .post(`/api/quiz_sessions/${sessionId}/join`)
    .send(payload)
    .expect(201)
  const body: ICreateParticipantApiResponse = response.body

  expect(body.id).toMatch(new RegExp(commonRegex.uuid))
}

export const startCountdown = async (
  api: TestAgent,
  auth: string,
  sessionId: string,
  start: number,
) => {
  const payload: IUpdateQuizSessionApiRequest = {
    start_time: start,
  }
  await api
    .patch(`/api/quiz_sessions/${sessionId}`)
    .set('Authorization', auth)
    .send(payload)
    .expect(200)
}

export const startQuiz = async (
  api: TestAgent,
  auth: string,
  sessionId: string,
  questionTime: number,
) => {
  const payload: IUpdateQuizSessionApiRequest = {
    status: QuizSessionStatus.Active,
    question_index: 0,
    question_end_time: questionTime,
  }
  await api
    .patch(`/api/quiz_sessions/${sessionId}`)
    .set('Authorization', auth)
    .send(payload)
    .expect(200)
}

export const endQuiz = async (api: TestAgent, auth: string, sessionId: string) => {
  const payload: IUpdateQuizSessionApiRequest = {
    status: QuizSessionStatus.Complete,
    end_time: Date.now(),
  }
  await api
    .patch(`/api/quiz_sessions/${sessionId}`)
    .set('Authorization', auth)
    .send(payload)
    .expect(200)
}

export const nextQuestion = async (
  api: TestAgent,
  auth: string,
  sessionId: string,
  index: number,
  endTime: number,
) => {
  const payload: IUpdateQuizSessionApiRequest = {
    question_index: index,
    question_end_time: endTime,
  }
  await api
    .patch(`/api/quiz_sessions/${sessionId}`)
    .set('Authorization', auth)
    .send(payload)
    .expect(200)
}

export const createResponse = async (
  api: TestAgent,
  auth: string,
  payload: ICreateQuizResponseApiRequest,
) => {
  await api
    .post('/api/quiz_responses')
    .set('Authorization', auth)
    .send(payload)
    .expect(201)
}
