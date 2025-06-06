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
import { quizBackgroundUrl } from '@frontend/util/ui'

const quizActive = computed(() => {
  return quizSession.value?.status === QuizSessionStatus.Active
})

const isHost = computed(() => {
  return store.auth.userId.value === quizSession.value?.user_id
})

const bgUrl = computed(() => {
  return quizActive.value ? undefined : quizBackgroundUrl(quizSession.value?.quiz)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-session {
  min-height: 100vh;
  background-size: cover;
  background-position: center;
  align-items: center;
  width: 100%;
}
</style>
