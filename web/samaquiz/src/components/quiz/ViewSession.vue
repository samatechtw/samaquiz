<template>
  <div class="view-session">
    <div v-if="state.loading" class="spinner-wrap f-center-col">
      <Spinner :size="32" />
    </div>
    <div v-else-if="state.session" class="quiz-wrap">
      <div class="session-top">
        <div class="session-title">
          <router-link :to="backLink" class="back-link">
            <Back class="back" />
          </router-link>
          <div>{{ ts('session.session') }}</div>
        </div>
        <div class="session-info">
          <div class="info-top">
            <div class="title-wrap">
              <div class="quiz-title">
                {{ state.session.quiz.title }}
              </div>
              <router-link
                :to="{ name: 'QuizSession', params: { code: state.session.code } }"
                target="_blank"
                class="view-wrap"
                @click.stop
              >
                <div class="view">
                  {{ ts('view') }}
                </div>
              </router-link>
            </div>
            <div class="session-status">
              {{ state.session.status }}
            </div>
          </div>
          <div class="info-row">
            <div class="info-entry">
              <div class="label">
                {{ ts('code') + ':' }}
              </div>
              <div class="entry">
                {{ state.session.code }}
              </div>
            </div>
            <div class="info-entry">
              <div class="label">
                {{ ts('started') + ':' }}
              </div>
              <div class="entry">
                {{ formatDate(state.session.start_time) }}
              </div>
            </div>
          </div>
          <div class="info-row">
            <div class="info-entry">
              <div class="label">
                {{ ts('participants') + ':' }}
              </div>
              <div class="entry">
                {{ state.session.participants?.length ?? 0 }}
              </div>
            </div>
            <div class="info-entry">
              <div class="label">
                {{ ts('ended') + ':' }}
              </div>
              <div class="entry">
                {{ formatDate(state.session.end_time) }}
              </div>
            </div>
          </div>
        </div>
        <div class="results-title">
          {{ ts('results') }}
        </div>
        <div class="participants">
          <div class="participant-wrap label-wrap">
            <div class="leader-avatar leader-label">
              {{ ts('avatar') }}
            </div>
            <div class="leader-name leader-label">
              {{ ts('name') }}
            </div>
            <div class="leader-points leader-label">
              {{ ts('points') }}
            </div>
          </div>
          <div v-for="leader in leadersState.leaders" class="participant-wrap">
            <div class="leader-avatar">
              <Avatar :url="leader.avatar" size="32" />
            </div>
            <div class="leader-name">
              {{ leader.name }}
            </div>
            <div class="leader-points">
              {{ leader.points }}
            </div>
          </div>
        </div>
      </div>
    </div>
    <NotFound v-else class="no-session" />
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, reactive } from 'vue'
import { useRoute } from 'vue-router'
import { Back } from '@frontend/components/svg'
import { Avatar, NotFound, Spinner } from '@frontend/components/widgets'
import {
  getLeaders,
  getQuizSession,
  IGetLeadersParams,
  IGetQuizSessionParams,
} from '@frontend/features'
import { ts } from '../../i18n'

const leadersState = reactive<IGetLeadersParams>({
  loading: false,
  leaders: undefined,
  error: undefined,
})

const route = useRoute()

const state: IGetQuizSessionParams = reactive({
  error: undefined,
  loading: false,
  session: undefined,
})

const formatDate = (date: number | string | undefined) => {
  if (!date) return '--'
  const d = new Date(date)
  const hours = d.getHours().toString().padStart(2, '0')
  const minutes = d.getMinutes().toString().padStart(2, '0')
  return `${d.getFullYear()}/${d.getMonth() + 1}/${d.getDate()} ${hours}:${minutes}`
}

const backLink = computed(() => {
  if (route.query?.fromQuizId) {
    return { name: 'QuizSessions', params: { id: route.query?.fromQuizId } }
  } else {
    return { name: 'Sessions' }
  }
})

onMounted(async () => {
  const code = route.params.code as string
  await getQuizSession(code, state)
  await getLeaders(state.session?.id, leadersState)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.view-session {
  padding-bottom: 80px;
  background-color: $color5;
}
.session-content {
  min-height: calc(100vh - $header-height);
}
.spinner-wrap {
  min-height: calc(100vh - $header-height);
}

.session-top {
  max-width: 720px;
  margin: 0 auto;
  padding-top: 40px;
}
.session-title {
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
.session-info {
  border: 1px solid $border1;
  border-radius: 16px;
  box-shadow:
    rgba(0, 0, 0, 0) 0px 0px 0px 0px,
    rgba(0, 0, 0, 0) 0px 0px 0px 0px,
    rgba(0, 0, 0, 0.05) 0px 1px 2px 0px;
  margin-top: 48px;
  padding: 24px 28px;
}
.info-top {
  display: flex;
  justify-content: space-between;
  margin-bottom: 24px;
}
.title-wrap {
  display: flex;
  align-items: center;
}
.quiz-title {
  @mixin title-regular 24px;
  margin-right: 16px;
}
.session-status {
  @mixin text 18px;
}
.info-row {
  margin-top: 20px;
  display: flex;
  justify-content: space-between;
  max-width: 85%;
}
.info-entry {
  display: flex;
}
.label {
  @mixin title-semi 15px;
  margin-right: 6px;
}
.entry {
  @mixin title-regular 15px;
}
.results-title {
  @mixin title 18px;
  margin-top: 40px;
  border-bottom: 1px solid $border1;
  padding-bottom: 4px;
}
.participants {
  display: flex;
  flex-direction: column;
  align-items: center;
  color: $text1;
  width: 100%;
}
.participant-wrap {
  @mixin list-item;
  margin: 0;
  padding: 6px 0;
  border-top: 1px solid $border2;
  align-items: center;
  .leader-label {
    @mixin title 16px;
  }
  &.label-wrap {
    border: none;
    margin-top: 16px;
  }
}
.leader-avatar {
  display: flex;
  align-items: center;
  margin-left: 8px;
  flex-shrink: 0;
  width: 15%;
}
.leader-name {
  @mixin title-regular 14px;
  min-width: 100px;
  width: 75%;
}
.leader-points {
  @mixin title 16px;
  min-width: 100px;
  width: 10%;
  text-align: center;
}
.no-participants {
  @mixin title-regular 18px;
  margin-top: 24px;
  color: $color2;
}
</style>
