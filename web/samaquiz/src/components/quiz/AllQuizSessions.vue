<template>
  <div class="quiz-sessions">
    <SessionsTable :sessions="sessions" />
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { sortQuizSessions } from '@frontend/features'
import SessionsTable from './SessionsTable.vue'

const sessions = computed(() => {
  return sortQuizSessions([])
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-sessions {
  min-height: calc(100vh - 380px);
}
</style>
