<template>
  <div class="quiz-host-complete">
    <div class="quiz-complete f-center-col">
      <div class="title">
        {{ ts('session.complete') }}
      </div>
      <QuizLeaders :leaders="leaders" :loadingLeaders="loadingLeaders" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { quizSession } from '@frontend/features'
import { apiGetSessionLeaders } from '@frontend/api'
import { ISessionLeader } from '@frontend/types'
import { ts } from '../../i18n'
import QuizLeaders from './QuizLeaders.vue'

const leaders = ref<ISessionLeader[]>()
const loadingLeaders = ref(false)

const showResults = async () => {
  if (!quizSession.value) {
    return
  }
  loadingLeaders.value = true
  try {
    const res = await apiGetSessionLeaders(quizSession.value.id)
    leaders.value = res.leaders
  } catch (e) {
    console.log('Failed to load leaders', e)
  }
  loadingLeaders.value = false
}

onMounted(() => {
  showResults()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-host-complete {
  padding-top: 100px;
  max-width: 800px;
  width: 100%;
  margin: 0 auto;
}
.quiz-complete {
  @mixin quiz-bubble;
  width: 100%;
}
.title {
  @mixin title 48px;
}
</style>
