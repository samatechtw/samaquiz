<template>
  <div class="quiz-sessions">
    <SessionsTable :sessions="sessions" :fromQuizId="quizState.quiz?.id" />
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { quizState, sortQuizSessions } from '@frontend/features'
import SessionsTable from './SessionsTable.vue'

const sessions = computed(() => {
  return sortQuizSessions(quizState.quiz?.sessions)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-sessions {
  min-height: calc(100vh - 380px);
}
</style>
