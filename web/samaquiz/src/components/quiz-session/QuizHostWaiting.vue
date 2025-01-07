<template>
  <div v-if="quizSession" class="quiz-session-title-wrap">
    <div class="quiz-session-title">
      <div class="title-wrap f-center-col">
        <div class="title">
          {{ quizSession.quiz.title }}
        </div>
        <div class="hosted">
          {{ ts('session.hosted') }}
        </div>
        <div class="host">
          <Avatar :url="quizSession.host_avatar" size="28" />
          <div class="host-name">
            {{ quizSession.host_name }}
          </div>
        </div>
        <div class="joined-wrap">
          <div class="count">
            {{ participantCount }}
          </div>
          <div>
            {{ ts('session.joined') }}
          </div>
        </div>
        <ErrorMessage :text="error" />
        <AppButton
          v-if="!isCountdown"
          :text="ts('start')"
          :animate="loading"
          class="start"
          @click="startQuizCountdown"
        />
      </div>
      <QrCode :url="url" :size="240" class="qr">
        <div class="qr-hover overlay f-center-col" @click="showQrModal = true">
          <div class="qr-text">
            {{ ts('session.qr_expand') }}
          </div>
        </div>
      </QrCode>
    </div>
    <Countdown
      v-if="isCountdown"
      v-model="countdown"
      :text="ts('session.countdown')"
      class="host-countdown"
      @complete="startQuiz"
    />
    <QrCodeModal :show="showQrModal" :url="url" @cancel="showQrModal = false" />
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import {
  AppButton,
  Avatar,
  Countdown,
  ErrorMessage,
  QrCode,
} from '@frontend/components/widgets'
import { WEB_URL } from '@frontend/util/config'
import { quizSession } from '@frontend/features'
import { QuizSessionStatus } from '@frontend/types'
import { apiUpdateSession } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import QrCodeModal from './QrCodeModal.vue'
import { ts } from '../../i18n'

defineProps<{
  participantCount: number
}>()

const url = `${WEB_URL}/q/${quizSession.value?.code}`

const showQrModal = ref(false)
const loading = ref(false)
const error = ref()
const countdown = ref()

const isCountdown = computed(() => {
  return countdown.value !== undefined && quizSession.value?.start_time !== null
})

const startQuizCountdown = async () => {
  if (quizSession.value) {
    loading.value = true
    try {
      const start = Date.now() + 1000 * 6
      await apiUpdateSession(quizSession.value.id, { start_time: start })
      quizSession.value.start_time = start
      countdown.value = Math.round((start - Date.now()) / 1000)
    } catch (e) {
      error.value = errorToKey(e)
    }
    loading.value = false
  }
}

const startQuiz = async () => {
  if (quizSession.value) {
    loading.value = true
    try {
      const endTime = Date.now() + quizSession.value.question_duration
      await apiUpdateSession(quizSession.value.id, {
        status: QuizSessionStatus.Active,
        question_index: 0,
        question_end_time: endTime,
      })
      quizSession.value.status = QuizSessionStatus.Active
      quizSession.value.question_index = 0
      quizSession.value.question_end_time = endTime
    } catch (e) {
      error.value = errorToKey(e)
    }
    loading.value = false
  }
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-session-title {
  @mixin quiz-bubble;
  padding: 24px 48px;
  width: 640px;
  display: flex;
  margin: 120px auto 0;
  position: relative;
  z-index: 10;
}
.title-wrap {
  max-width: 320px;
  width: 320px;
}
.title {
  @mixin title 32px;
}
.hosted {
  @mixin title-regular 14px;
  margin-top: 8px;
}
.host {
  @mixin title 16px;
  display: flex;
  align-items: center;
  margin-top: 8px;
}
.host-name {
  margin-left: 6px;
}
.qr-hover {
  opacity: 0;
  transition: opacity 0.2s linear;
  background-color: rgba(255, 255, 255, 0.9);
  user-select: none;
  cursor: pointer;
}
.qr-text {
  @mixin title 18px;
}
.qr:hover {
  .qr-hover {
    opacity: 1;
  }
}

.quiz-host-waiting {
  position: relative;
  padding-top: 24px;
}
.joined-wrap {
  @mixin title 24px;
  display: flex;
  align-items: center;
  position: relative;
  margin-top: 24px;
}
.count {
  margin: -14px 12px 0 0;
  font-size: 50px;
}
.start {
  margin-top: 16px;
}
.host-countdown {
  margin: 24px auto 0;
}
</style>
