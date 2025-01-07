<template>
  <div class="quiz-session-participant f-center-col">
    <div v-if="loading" class="spinner-wrap f-center">
      <Spinner :size="24" color="#3282b8" />
    </div>
    <div v-else-if="error" class="error-wrap f-center-col">
      <div>
        {{ error }}
      </div>
      <div class="reload">
        {{ ts('session.reload') }}
      </div>
      <div class="new-user" @click="clear">
        {{ ts('session.user') }}
      </div>
    </div>
    <QuizWaiting
      v-else-if="localParticipant && quizSession?.status === QuizSessionStatus.Ready"
      :localParticipant="localParticipant"
    >
      <Countdown v-if="countdown !== undefined" v-model="countdown" />
    </QuizWaiting>
    <QuizActive
      v-else-if="localParticipant && quizSession?.status === QuizSessionStatus.Active"
      :localParticipant="localParticipant"
    />
    <QuizRegister v-else />
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, ref } from 'vue'
import { store } from '@frontend/store'
import { errorToKey } from '@frontend/util/api'
import {
  IGetParticipantApiResponse,
  IWsServerMessage,
  QuizSessionStatus,
  WsServerMessageType,
} from '@frontend/types'
import { Countdown, Spinner } from '@frontend/components/widgets'
import { quizSession } from '@frontend/features'
import { apiGetParticipant } from '@frontend/api'
import { ServerMessageHandler, useWebsocket } from '@frontend/websocket'
import QuizWaiting from './QuizWaiting.vue'
import QuizActive from './QuizActive.vue'
import QuizRegister from './QuizRegister.vue'
import { ts } from '../../i18n'

const error = ref()
const loading = ref(true)
const participant = ref<IGetParticipantApiResponse>()
const countdown = ref()

const setNextQuestion = (index: number, endTime: number) => {
  countdown.value = Math.round((endTime - Date.now()) / 1000)
  if (quizSession.value) {
    quizSession.value.question_index = index
    quizSession.value.question_end_time = endTime
  }
}

const handleMessage: ServerMessageHandler = (msg: IWsServerMessage) => {
  switch (msg.type) {
    case WsServerMessageType.QuizCountdown:
      const countdownMs = (msg.value as number) - Date.now()
      countdown.value = Math.round(countdownMs / 1000)
      break
    case WsServerMessageType.QuizStart:
      setNextQuestion(msg.question_index as number, msg.question_end_time as number)
      break
    case WsServerMessageType.QuestionStart:
      setNextQuestion(msg.question_index as number, msg.question_end_time as number)
      break
  }
}

useWebsocket(quizSession.value?.id as string, handleMessage)

const localParticipant = computed(() => {
  if (quizSession.value) {
    return store.participant.sessions.value[quizSession.value.id]
  }
})

const load = async () => {
  const participantId = localParticipant.value?.participantId
  if (participantId) {
    try {
      participant.value = await apiGetParticipant(participantId)
    } catch (e) {
      if ((e as any).status === 404) {
        error.value = ts('errors.ParticipantNotFound')
      } else {
        error.value = ts(errorToKey(e))
      }
    }
  }
  loading.value = false
}

const clear = () => {
  error.value = undefined
  if (quizSession.value) {
    store.participant.clearParticipant(quizSession.value.id)
  }
}

onMounted(async () => {
  await load()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-session-participant {
  width: 100%;
}
.spinner-wrap {
  padding-top: 24px;
}
.error-wrap {
  @mixin quiz-bubble;
  @mixin title-regular 18px;
  width: 300px;
  margin-top: 120px;
  color: $red;
}
.reload {
  margin: 8px 0 16px;
}
.new-user {
  color: $text2;
  cursor: pointer;
  text-decoration: underline;
  font-size: 15px;
  user-select: none;
}
</style>
