<template>
  <div class="quiz-sessions">
    <SessionsTable :sessions="sessions" />
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, reactive } from 'vue'
import {
  IListQuizSessionParams,
  listQuizSessions,
  sortQuizSessions,
} from '@frontend/features'
import SessionsTable from './SessionsTable.vue'
import { store } from '@frontend/store'

const state: IListQuizSessionParams = reactive({
  error: undefined,
  loading: false,
  quiz_sessions: [],
})

const sessions = computed(() => {
  return sortQuizSessions(state.quiz_sessions)
})

onMounted(() => {
  listQuizSessions({ user_id: store.auth.userId.value }, state)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-sessions {
  min-height: calc(100vh - 380px);
}
</style>
