<template>
  <div class="quiz-session f-col" :style="{ 'background-image': bgUrl }">
    <QuizSessionHost v-if="isHost" />
    <QuizSessionParticipant v-else />
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { quizSession } from '@frontend/features'
import { QuizSessionStatus } from '@frontend/types'
import { store } from '@frontend/store'
import QuizSessionParticipant from './QuizSessionParticipant.vue'
import QuizSessionHost from './QuizSessionHost.vue'

const quizActive = computed(() => {
  return quizSession.value?.status === QuizSessionStatus.Active
})

const isHost = computed(() => {
  return store.auth.userId.value === quizSession.value?.user_id
})

const bgUrl = computed(() => {
  const bg = quizSession.value?.quiz?.intro_background_url
  if (quizActive.value || !bg) {
    return undefined
  }
  return `url(${bg})`
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-session {
  min-height: 100vh;
  background-size: cover;
  background-position: 50% 10%;
  align-items: center;
  width: 100%;
}
</style>
