<template>
  <div class="quiz-session-host">
    <QuizHostWaiting v-if="isReady" :participantCount="participantCount" />
    <QuizHostActive
      v-else-if="quizSession?.status === QuizSessionStatus.Active"
      :responseCount="responseCount"
    />
    <QuizHostComplete v-else-if="quizSession?.status === QuizSessionStatus.Complete" />
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { IWsServerMessage, QuizSessionStatus, WsServerMessageType } from '@frontend/types'
import QuizHostActive from './QuizHostActive.vue'
import QuizHostWaiting from './QuizHostWaiting.vue'
import { ServerMessageHandler, useWebsocket } from '@frontend/websocket'
import { quizSession } from '@frontend/features'
import QuizHostComplete from './QuizHostComplete.vue'

const liveParticipants = ref()
const responseCount = ref()

const isReady = computed(() => {
  return quizSession.value?.status === QuizSessionStatus.Ready
})

const handleMessage: ServerMessageHandler = (msg: IWsServerMessage) => {
  switch (msg.type) {
    case WsServerMessageType.Joined:
      liveParticipants.value = msg.value as number
      break
    case WsServerMessageType.QuizCountdown:
      break
    case WsServerMessageType.QuizResponse:
      responseCount.value = msg.value as number
      break
    case WsServerMessageType.QuestionStart:
      responseCount.value = 0
      break
  }
}

useWebsocket(quizSession.value?.id as string, handleMessage)

const participantCount = computed(() => {
  return liveParticipants.value ?? quizSession.value?.participants?.length ?? 0
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-session-host {
  width: 100%;
  padding-bottom: 120px;
}
</style>
