<template>
  <div class="edit-question-page">
    <ViewSession />
  </div>
</template>

<script lang="ts" setup>
import { ViewSession } from '@frontend/components/quiz'
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.edit-question-page {
  min-height: calc(100vh - $header-height);
  padding-top: 140px;
  padding-bottom: 80px;
  background-color: $color5;
}
</style>
