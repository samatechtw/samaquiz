<template>
  <div class="quiz-host-complete">
    <div class="quiz-complete f-center-col">
      <div class="title">
        {{ ts('session.complete') }}
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'
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
