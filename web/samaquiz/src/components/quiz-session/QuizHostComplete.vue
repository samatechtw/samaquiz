<template>
  <div class="quiz-host-complete">
    <div class="quiz-complete f-center-col">
      <div class="title">
        {{ ts('session.complete') }}
      </div>
      <QuizLeaders :leaders="state.leaders" :loadingLeaders="state.loading" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive } from 'vue'
import { quizSession, getLeaders, IGetLeadersParams } from '@frontend/features'
import { ts } from '../../i18n'
import QuizLeaders from './QuizLeaders.vue'

const state = reactive<IGetLeadersParams>({
  loading: false,
  leaders: undefined,
  error: undefined,
})

const showResults = async () => {
  await getLeaders(quizSession.value?.id, state)
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
  padding-bottom: 40px;
  width: 100%;
}
.title {
  @mixin title 48px;
}
</style>
