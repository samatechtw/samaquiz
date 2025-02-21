<template>
  <div class="quiz-question-wrap">
    <div v-if="loading" class="loading-wrap f-center-col">
      <Spinner :size="48" />
    </div>
    <div v-else-if="question" class="quiz-question f-center-col">
      <div v-if="showResults" class="question-results f-center-col">
        <div
          v-if="participantCount && participantCount > 0"
          class="result-count f-center"
        >
          {{ ts('responses') }}
          <span>
            {{ `${responseCount ?? 0} / ${participantCount}` }}
          </span>
          <AppButton :text="ts('next')" @click="nextQuestion" class="results-next" />
        </div>
        <QuizLeaders :leaders="leaders" :loadingLeaders="loadingLeaders" />
      </div>
      <Countdown
        v-else-if="countdown !== undefined"
        v-model="countdown"
        class="question-countdown"
        @complete="countdownComplete"
      >
        <div class="info-wrap">
          <div class="info-buttons f-col">
            <AppButton :text="ts('results')" @click="setShowResults" />
            <AppButton
              v-if="countdown"
              :text="ts('extend')"
              @click="extendQuestion"
              class="extend"
            />
          </div>
          <div v-if="participantCount && participantCount > 0" class="response-count">
            <div class="count">
              {{ `${responseCount ?? 0} / ${participantCount}` }}
            </div>
          </div>
        </div>
      </Countdown>
      <div class="text">
        {{ question.text }}
      </div>
      <div class="answers">
        <QuizAnswer
          v-for="(answer, index) in question.answers"
          :text="answer.text"
          :index="index"
          :dim="showResults && !answer.is_correct"
          :correct="showResults && answer.is_correct"
        />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue'
import {
  IGetQuestionApiResponse,
  ISessionLeader,
  QuizSessionStatus,
} from '@frontend/types'
import { AppButton, Spinner } from '@frontend/components/widgets'
import { errorToKey } from '@frontend/util/api'
import { apiGetQuestion, apiGetSessionLeaders, apiUpdateSession } from '@frontend/api'
import { ts } from '../../i18n'
import QuizAnswer from './QuizAnswer.vue'
import Countdown from '../widgets/Countdown.vue'
import { quizSession } from '@frontend/features'
import QuizLeaders from './QuizLeaders.vue'

const question = ref<IGetQuestionApiResponse>()
const loading = ref(false)
const showResults = ref(false)
const error = ref()
const countdown = ref()
const leaders = ref<ISessionLeader[]>()
const loadingLeaders = ref(false)

const { questionId } = defineProps<{
  questionId: string
  participantCount: number | undefined
  responseCount: number | undefined
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
  try {
    question.value = await apiGetQuestion(questionId)
    updateCountdown()
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
}

const setShowResults = async () => {
  if (!quizSession.value) {
    return
  }
  showResults.value = true
  loadingLeaders.value = true
  try {
    const res = await apiGetSessionLeaders(quizSession.value.id)
    leaders.value = res.leaders
  } catch (e) {
    console.log('Failed to load leaders', e)
  }
  loadingLeaders.value = false
}

const extendQuestion = async () => {
  const questionEnd = quizSession.value?.question_end_time
  if (loading.value || !quizSession.value || !questionEnd) {
    return
  }
  loading.value = true
  try {
    const extendedTime = questionEnd + 1000 * 10
    await apiUpdateSession(quizSession.value.id, {
      question_end_time: extendedTime,
    })
    quizSession.value.question_end_time = extendedTime
    updateCountdown()
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  loading.value = false
}

const nextQuestion = async () => {
  if (loading.value || !quizSession.value) {
    return
  }
  loading.value = true
  const { id, question_duration, question_index } = quizSession.value
  const nextIndex = question_index === undefined ? 0 : question_index + 1
  const now = Date.now()
  const nextTime = now + question_duration + 1000
  try {
    if (nextIndex < quizSession.value.quiz.questions.length) {
      await apiUpdateSession(id, {
        question_index: nextIndex,
        question_end_time: nextTime,
      })
      quizSession.value.question_index = nextIndex
      quizSession.value.question_end_time = nextTime
    } else {
      await apiUpdateSession(id, {
        end_time: now,
        status: QuizSessionStatus.Complete,
      })
      quizSession.value.end_time = now
      quizSession.value.status = QuizSessionStatus.Complete
    }
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  showResults.value = false
  loading.value = false
}

const countdownComplete = () => {
  setShowResults()
}

watch(() => questionId, getQuestion)

onMounted(() => {
  getQuestion()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-question-wrap {
  width: 100%;
}
.quiz-question {
  width: 100%;
  .question-countdown {
    display: flex;
    flex-direction: row;
    width: auto;
    min-width: 300px;
  }
}
.loading-wrap {
  padding-top: 120px;
}
.text {
  @mixin question-text;
}
.answers {
  display: flex;
  flex-wrap: wrap;
  justify-content: space-between;
  width: 100%;
  margin-top: 40px;
}
.info-wrap {
  margin-left: 40px;
}
.question-results {
  @mixin quiz-bubble;
  width: 600px;
}
.response-count {
  font-size: 20px;
  margin-top: 16px;
}
.next {
  margin-top: 24px;
}
.info-buttons {
}
.extend {
  margin-top: 12px;
}
.results-next {
  margin-left: 24px;
}
.result-count {
  @mixin title-regular 20px;
  span {
    font-weight: bold;
    font-size: 22px;
    margin-left: 16px;
  }
}
.leaders {
  width: 100%;
}
.leaders-title {
  @mixin title 20px;
  margin-top: 16px;
}
.leader-view {
  display: flex;
  align-items: center;
  width: 50%;
}
.leader-name {
  @mixin title 20px;
  margin: 0 12px 0 8px;
}
.leader-index {
  @mixin title 18px;
}
.leader-avatar {
  margin-left: 8px;
}
.leader-points {
  @mixin title 24px;
}
</style>
