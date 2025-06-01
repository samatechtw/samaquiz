<template>
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
  </div>
  <div v-else class="no-sessions">
    {{ ts('session.empty') }}
  </div>
</template>

<script lang="ts" setup>
import { formatDistance } from 'date-fns'
import { IQuizSessionViewModel } from '@frontend/types'
import { ts } from '../../i18n'

defineProps<{
  sessions: IQuizSessionViewModel[]
}>()

const dateStr = (time: number | null | undefined) => {
  if (!time) {
    return '-'
  }
  formatDistance(new Date(time), new Date(), { addSuffix: true })
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

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
.no-sessions {
  @mixin title-regular 18px;
  margin-top: 24px;
  color: $color2;
}
</style>
