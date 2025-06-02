<template>
  <div class="quiz-top">
    <div class="quiz-title">
      <router-link :to="{ name: 'Quizzes' }" class="back-link">
        <Back class="back" />
      </router-link>
      <div>{{ title }}</div>
      <div class="export" @click="emit('export')">
        {{ ts('export') }}
      </div>
      <AppButton
        :text="ts('session.new')"
        class="new-session"
        @click="quizState.newSession = NewSessionStatus.Show"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { NewSessionStatus, quizState } from '@frontend/features'
import { AppButton } from '@frontend/components/widgets'
import { Back } from '@frontend/components/svg'
import { ts } from '../../i18n'

const route = useRoute()

const emit = defineEmits<{
  (e: 'export'): void
}>()

const title = computed(() => {
  if (route.name === 'QuizSessions') {
    return ts('quiz.sessions')
  }
  return ts('quiz.edit')
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-top {
  max-width: 720px;
  margin: 0 auto;
  padding-top: 40px;
}
.quiz-title {
  @mixin title-extra 24px;
  display: flex;
  align-items: center;
}
.back {
  @mixin size 24px;
}
.back-link {
  display: flex;
  margin-right: 12px;
}
.export {
  @mixin title-regular 16px;
  margin: 0 24px 0 auto;
  text-decoration: underline;
  user-select: none;
  cursor: pointer;
}
</style>
