import {
  ICreateParticipantApiRequest,
  IWsClientMessage,
  IWsServerMessage,
  UserType,
  WsClientMessageType,
  WsReceiverType,
  WsServerMessageType,
} from '@frontend/types'
import request from 'superwstest'
import { afterEach, beforeAll, beforeEach, describe, test } from 'vitest'
import { AppDbResetService } from '@test/shared'
import { testConfig } from '../test.config'
import {
  createParticipant,
  createResponse,
  endQuiz,
  generateAuthToken,
  nextQuestion,
  startCountdown,
  startQuiz,
  TestAgent,
  testagent,
} from '../helpers'
import { commonRegex } from '@frontend/util/format'

// Hack to make superwstest types work
type MType = Record<string, any>

const ws_joined = (count: number): MType => {
  const joined: IWsServerMessage = {
    type: WsServerMessageType.Joined,
    value: count,
  }
  return joined as MType
}

const ws_quiz_countdown = (time: number): MType => {
  const started: IWsServerMessage = {
    type: WsServerMessageType.QuizCountdown,
    value: time,
  }
  return started as MType
}

interface QuestionData {
  question_index: number
  question_end_time: number
}

const ws_quiz_start = (data: QuestionData): MType => {
  const started: IWsServerMessage = {
    type: WsServerMessageType.QuizStart,
    question_index: data.question_index,
    question_end_time: data.question_end_time,
  }
  return started as MType
}

const ws_question_start = (data: QuestionData): MType => {
  const questionStart: IWsServerMessage = {
    type: WsServerMessageType.QuestionStart,
    question_index: data.question_index,
    question_end_time: data.question_end_time,
  }
  return questionStart as MType
}

const ws_quiz_response = (count: number): MType => {
  const response: IWsServerMessage = {
    type: WsServerMessageType.QuizResponse,
    value: count,
  }
  return response as MType
}

const compare = (actual: unknown, msg: MType): boolean => {
  const result = JSON.stringify(actual) === JSON.stringify(msg)
  if (!result) {
    console.log(`Got ${JSON.stringify(actual)}, expected ${JSON.stringify(msg)}`)
  }
  return result
}

const compareQuestion = (actual: unknown, data: QuestionData): boolean => {
  const expected = ws_question_start(data)
  return compare(actual, expected)
}

const compareQuizStart = (actual: unknown, data: QuestionData): boolean => {
  const expected = ws_quiz_start(data)
  return compare(actual, expected)
}

const ws_quiz_end = (): MType => {
  const ended: IWsServerMessage = {
    type: WsServerMessageType.QuizEnd,
    value: 0,
  }
  return ended as MType
}

describe('Host a Quiz Session', () => {
  let testEndpoint = (sessionId: string) => `/api/ws/${sessionId}`
  let testHelperApiUrl: string
  let dbResetService: AppDbResetService
  let apiHost: string
  let api: TestAgent
  let createPayload: ICreateParticipantApiRequest
  let sessionId: string
  let hostAuth: string
  let hostToken: string
  let participantId: string

  beforeAll(() => {
    apiHost = testConfig.get('apiUrl')
    api = testagent(testConfig.get('apiUrl'))
    testHelperApiUrl = testConfig.get('apiTestHelperUrl')
    dbResetService = new AppDbResetService(testHelperApiUrl)
  })

  beforeEach(async () => {
    await dbResetService.resetDb()
    sessionId = 'ce782f8a-01bb-4228-9e50-4db0248f14cd'
    participantId = '9ee64573-c84f-439e-8cc7-a0b7b1f1906f'
    createPayload = {
      name: 'P1',
      avatar: 'http://localhost:8080/src/theme/img/cats/cat10.png',
    }
    hostToken = generateAuthToken('2213d9fc-3693-47ed-a495-cd5e7fc6dd0e', UserType.User)
    hostAuth = `Bearer ${hostToken}`
  })

  afterEach(() => {
    request.closeAll()
  })

  test('connects a host and client and complete quiz session', async () => {
    const participantReady: IWsServerMessage = {
      type: WsServerMessageType.Ready,
      value: WsReceiverType.Participant,
    }
    const hostReady: IWsServerMessage = {
      type: WsServerMessageType.Ready,
      value: WsReceiverType.Host,
    }
    const participantAuth: IWsClientMessage = {
      type: WsClientMessageType.Auth,
    }
    const hostAuthMsg: IWsClientMessage = {
      type: WsClientMessageType.Auth,
      value: hostToken,
    }
    const start = Date.now() + 1 * 1000
    let question1: QuestionData = {
      question_end_time: 0,
      question_index: 0,
    }
    let question2: QuestionData = {
      question_end_time: 0,
      question_index: 1,
    }
    let question3: QuestionData = {
      question_end_time: 0,
      question_index: 2,
    }
    let question4: QuestionData = {
      question_end_time: 0,
      question_index: 3,
    }
    const host = request(apiHost)
      .ws(testEndpoint(sessionId))
      .sendJson(hostAuthMsg as MType)
      .expectJson(hostReady as MType)
      .exec(async () => createParticipant(api, sessionId, createPayload))
      .expectJson(ws_joined(4))
      .exec(async () => startCountdown(api, hostAuth, sessionId, start))
      .expectJson(ws_quiz_countdown(start))
      .exec(async () => {
        question1.question_end_time = Date.now() + 30 * 1000
        await startQuiz(api, hostAuth, sessionId, question1.question_end_time)
      })
      .expectJson((actual) => compareQuizStart(actual, question1))
      .expectJson(ws_quiz_response(1))
      .exec(async () => {
        question2.question_end_time = Date.now() + 30 * 1000
        await nextQuestion(api, hostAuth, sessionId, 1, question2.question_end_time)
      })
      .expectJson((actual) => compareQuestion(actual, question2))
      .expectJson(ws_quiz_response(1))
      .exec(async () => {
        question3.question_end_time = Date.now() + 30 * 1000
        await nextQuestion(api, hostAuth, sessionId, 2, question3.question_end_time)
      })
      .expectJson((actual) => compareQuestion(actual, question3))
      .expectJson(ws_quiz_response(1))
      .exec(async () => {
        question4.question_end_time = Date.now() + 30 * 1000
        await nextQuestion(api, hostAuth, sessionId, 3, question4.question_end_time)
      })
      .expectJson((actual) => compareQuestion(actual, question4))
      .expectJson(ws_quiz_response(1))
      .exec(async () => endQuiz(api, hostAuth, sessionId))
      .expectJson(ws_quiz_end())
      .close()
    const participant1 = request(apiHost)
      .ws(testEndpoint(sessionId))
      .sendJson(participantAuth as MType)
      .expectJson(participantReady as MType)
      .expectJson(ws_quiz_countdown(start))
      .expectJson((actual) => compareQuizStart(actual, question1))
      // Correct
      .exec(async () =>
        createResponse(api, hostAuth, {
          participant_id: participantId,
          question_id: '11354d45-903d-4493-9b96-5f07497b01e1', // Q1
          answer_id: 'f424b181-ec35-4a3b-9b81-d467cd0cbe7b', // Q1A1 (Correct)
        }),
      )
      .expectJson((actual) => compareQuestion(actual, question2))
      .exec(async () =>
        createResponse(api, hostAuth, {
          participant_id: participantId,
          question_id: '813a13c9-4562-4fa3-8d23-f46a079a57de', // Q2
          answer_id: 'd06315f6-6304-45c0-9020-983b232e4701', // Q2A2 (Correct)
        }),
      )
      .expectJson((actual) => compareQuestion(actual, question3))
      .exec(async () =>
        createResponse(api, hostAuth, {
          participant_id: participantId,
          question_id: 'a82c0a88-10eb-43cd-b057-7214bb598111', // Q3
          answer_id: '5b857643-f296-42eb-b65c-4adddde33ca3', // Q3A2 (Correct)
        }),
      )
      .expectJson((actual) => compareQuestion(actual, question4))
      .exec(async () =>
        createResponse(api, hostAuth, {
          participant_id: participantId,
          question_id: '4f7b5fba-7c60-4e39-b0e2-58bd8f6aa1cd', // Q4
          answer_id: 'a8685eee-6d99-41e5-b4c5-c1711c5fc5cb', // Q4A1 (Incorrect)
        }),
      )
      .expectJson(ws_quiz_end())
      .close()

    await Promise.all([host, participant1])
  })
})
