<template>
  <div class="quiz-host-active">
    <QuizHostQuestion
      v-if="questionId"
      :questionId="questionId"
      :participantCount="participantCount"
      :responseCount="responseCount"
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
import { computed, onMounted, ref } from 'vue'
import { quizSession } from '@frontend/features'
import { ts } from '../../i18n'
import QuizHostQuestion from './QuizHostQuestion.vue'
import { apiGetParticipantCount } from '@frontend/api'

defineProps<{
  responseCount: number | undefined
}>()

const participantCount = ref()

const questionId = computed(() => {
  const index = quizSession.value?.question_index
  if (index === undefined) {
    return
  }
  return quizSession.value?.quiz.questions_order[index]
})

const getParticipantCount = async () => {
  if (!quizSession.value) {
    return
  }
  try {
    const res = await apiGetParticipantCount(quizSession.value.id)
    participantCount.value = res.count
  } catch (e) {
    console.log('Failed to get participant count')
  }
}

onMounted(() => {
  getParticipantCount()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-host-active {
  max-width: 800px;
  width: 100%;
  margin: 0 auto;
  padding: 16px 0 120px;
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
</style>
