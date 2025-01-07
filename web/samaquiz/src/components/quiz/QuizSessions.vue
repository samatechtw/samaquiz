<template>
  <div class="quiz-sessions">
    <div v-if="sessions.length" class="sessions-wrap">
      <div class="session-head">
        <div class="code">
          {{ ts('code') }}
        </div>
        <div class="status">
          {{ ts('status') }}
        </div>
        <div class="start">
          {{ ts('started') }}
        </div>
        <div class="end">
          {{ ts('ended') }}
        </div>
      </div>
      <div class="sessions">
        <div v-for="session in sessions" class="session">
          <div class="code">
            {{ session.code }}
          </div>
          <div class="status">
            {{ session.status }}
          </div>
          <div class="start">
            {{ dateStr(session.start_time) }}
          </div>
          <div class="end">
            {{ dateStr(session.end_time) }}
          </div>
          <router-link
            :to="{ name: 'QuizSession', params: { code: session.code } }"
            target="_blank"
            class="view-wrap"
          >
            <div class="view">
              {{ ts('view') }}
            </div>
          </router-link>
        </div>
      </div>
      <div v-if="!sessions.length" class="no-sessions">
        {{ ts('session.empty') }}
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { formatDistance } from 'date-fns'
import { quizState } from '@frontend/features'
import { ts } from '../../i18n'
import { computed } from 'vue'
import { QuizSessionStatus } from '@frontend/types'

const dateStr = (time: number | null | undefined) => {
  if (!time) {
    return '-'
  }
  formatDistance(new Date(time), new Date(), { addSuffix: true })
}

const statusOrder: Record<QuizSessionStatus, number> = {
  Ready: 3,
  Active: 4,
  Complete: 2,
  Canceled: 1,
}

const sessions = computed(() => {
  const quizSessions = quizState.quiz?.sessions ?? []
  return quizSessions.sort((a, b) => {
    if (a.status !== b.status) {
      return statusOrder[b.status] - statusOrder[a.status]
    }
    return (b.start_time ?? 0) - (a.start_time ?? 0)
  })
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-sessions {
  min-height: calc(100vh - 380px);
}
.session {
  @mixin list-item;
  padding: 12px 20px;
  border: 1px solid $border1;
}
.code {
  @mixin title-regular 14px;
  min-width: 100px;
}
.status {
  @mixin title 13px;
  min-width: 100px;
}
.start,
.end {
  @mixin title-regular 13px;
  min-width: 88px;
}
.view-wrap {
  margin-left: auto;
}
.view {
  @mixin title 13px;
  color: $color3;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-left: 8px;
}
.session-head {
  @mixin list-item;
  padding: 0 20px;
  > * {
    @mixin title 13px;
  }
}
</style>
