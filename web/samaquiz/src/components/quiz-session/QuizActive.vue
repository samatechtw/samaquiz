<template>
  <div class="quiz-host-active">
    <QuizQuestion
      v-if="questionId"
      :localParticipant="localParticipant"
      :questionId="questionId"
    />
    <div v-else class="no-question f-center-col">
      <div class="no-question-title">
        {{ ts('session.no_question') }}
      </div>
      <div class="no-question-text">
        {{ ts('session.no_question_text') }}
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import { quizSession } from '@frontend/features'
import QuizQuestion from './QuizQuestion.vue'
import { IQuizParticipant } from '@frontend/types'
import { ts } from '../../i18n'

defineProps<{
  localParticipant: IQuizParticipant
}>()

const questionId = computed(() => {
  const index = quizSession?.value?.question_index
  if (index === undefined) {
    return
  }
  return quizSession.value?.quiz.questions_order[index]
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-host-active {
  max-width: 800px;
  width: 100%;
  margin: 0 auto;
  padding: 16px 24px 120px;
}
.no-question {
  padding-top: 80px;
}
.no-question-title {
  @mixin title 36px;
}
.no-question-text {
  @mixin title-regular 20px;
  margin-top: 24px;
}
@media (max-width: 800px) {
  .quiz-host-active {
    max-width: 600px;
  }
}
</style>
