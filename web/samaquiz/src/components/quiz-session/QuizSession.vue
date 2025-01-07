<template>
  <div
    class="quiz-session f-col"
    :style="{ 'background-image': quizActive ? undefined : `url(${quizBg})` }"
  >
    <QuizSessionHost v-if="isHost" />
    <QuizSessionParticipant v-else />
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { quizSession } from '@frontend/features'
import { QuizSessionStatus } from '@frontend/types'
import { store } from '@frontend/store'
import defaultBg from '@theme/img/bg/bg1.jpg'
import QuizSessionParticipant from './QuizSessionParticipant.vue'
import QuizSessionHost from './QuizSessionHost.vue'

const quizBg = defaultBg

const quizActive = computed(() => {
  return quizSession.value?.status === QuizSessionStatus.Active
})

const isHost = computed(() => {
  return store.auth.userId.value === quizSession.value?.user_id
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
