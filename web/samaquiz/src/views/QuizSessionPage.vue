<template>
  <div class="quiz-session-page">
    <div v-if="loadingSession" class="spinner-wrap f-center">
      <Spinner :size="40" color="#3282b8" />
    </div>
    <div v-else-if="sessionError" class="error-wrap f-center-col">
      <div class="error-title">
        {{ ts('session.error') }}
      </div>
      <div class="error-text">
        {{ sessionError }}
      </div>
      <router-link :to="{ name: 'Profile' }" class="view-quizzes">
        <AppButton :text="ts('session.view_quizzes')" />
      </router-link>
    </div>
    <QuizSession v-else-if="quizSession" />
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import {
  getQuizSession,
  loadingSession,
  quizSession,
  sessionError,
} from '@frontend/features'
import { AppButton, Spinner } from '@frontend/components/widgets'
import { QuizSession } from '@frontend/components/quiz-session'
import { ts } from '../i18n'

const route = useRoute()

onMounted(async () => {
  const code = route.params.code as string
  await getQuizSession(code)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-session-page {
  min-height: 100vh;
}
.spinner-wrap {
  min-height: 100vh;
}
.error-wrap {
  padding-top: 140px;
}
.error-title {
  @mixin title 28px;
  color: $text1;
}
.error-text {
  @mixin title-regular 20px;
  margin-top: 24px;
  color: $text2;
}
.view-quizzes {
  margin-top: 32px;
}
</style>
