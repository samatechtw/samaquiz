<template>
  <div class="quiz-question-wrap">
    <div v-if="loading" class="loading-wrap f-center-col">
      <Spinner :size="48" />
    </div>
    <div
      v-else-if="question"
      class="quiz-question f-center-col"
      :class="{ 'has-asset': !!question.asset_url }"
    >
      <div v-if="answeredId || !countdown" class="wait-host">
        {{ ts('session.wait_host') }}
      </div>
      <Countdown
        v-else
        v-model="countdown"
        class="question-countdown"
        @complete="countdownComplete"
      >
      </Countdown>
      <div class="text" v-html="question.text" />
      <QuestionAsset :question="question" />
      <div class="answers">
        <QuizAnswer
          v-for="(answer, index) in question.answers"
          :text="answer.text"
          :index="index"
          :dim="answeredId && answeredId !== answer.id"
          :correct="answeredId && correctId === answer.id"
          :incorrect="answeredId && correctId !== answer.id && answeredId === answer.id"
          @click="answerQuestion(answer.id)"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue'
import { IGetQuestionApiResponse, IQuizParticipant } from '@frontend/types'
import { Spinner } from '@frontend/components/widgets'
import { errorToKey } from '@frontend/util/api'
import { apiCreateQuizResponse, apiGetQuestion } from '@frontend/api'
import { ts } from '../../i18n'
import QuizAnswer from './QuizAnswer.vue'
import Countdown from '../widgets/Countdown.vue'
import { quizSession } from '@frontend/features'
import QuestionAsset from './QuestionAsset.vue'

const question = ref<IGetQuestionApiResponse>()
const loading = ref(true)
const answering = ref(false)
const answeredId = ref()
const correctId = ref()
const error = ref()
const countdown = ref()

const { questionId, localParticipant } = defineProps<{
  questionId: string
  localParticipant: IQuizParticipant
}>()

const updateCountdown = () => {
  const questionEnd = quizSession.value?.question_end_time
  const now = Date.now()
  if (questionEnd && questionEnd > now) {
    countdown.value = Math.round((questionEnd - now) / 1000)
  } else {
    countdown.value = 0
  }
}

const getQuestion = async () => {
  answering.value = false
  answeredId.value = undefined
  correctId.value = undefined
  loading.value = true
  try {
    question.value = await apiGetQuestion(questionId)
    updateCountdown()
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  loading.value = false
}

const answerQuestion = async (answerId: string) => {
  if (answering.value || answeredId.value) {
    return
  }
  answering.value = true
  try {
    await apiCreateQuizResponse({
      participant_id: localParticipant.participantId,
      question_id: questionId,
      answer_id: answerId,
    })
    answeredId.value = answerId
    correctId.value = question.value?.answers.find((a) => a.is_correct)?.id
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  answering.value = false
}

const countdownComplete = () => {}

watch(
  () => questionId,
  () => {
    getQuestion()
  },
)

onMounted(() => {
  getQuestion()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-question-wrap {
  width: 100%;
}
.answers {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  width: 100%;
  margin-top: 40px;
}
.quiz-question {
  width: 100%;
  .question-countdown {
    width: 200px;
    padding: 16px 20px;
    :deep(.countdown) {
      font-size: 64px;
    }
  }
  &.has-asset {
    .text {
      margin-top: 20px;
      font-size: 40px;
    }
    .answers {
      margin-top: 20px;
    }
  }
}
.loading-wrap {
  padding-top: 120px;
}
.wait-host {
  @mixin quiz-bubble;
  @mixin title-regular 24px;
}
.text {
  @mixin question-text;
}
</style>
