<template>
  <div class="not-found content-wrap">
    <div class="not-found-title content-title">
      {{ ts('not_found') }}
    </div>
    <div class="not-found-text">
      {{ ts('not_found_text') }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../i18n'
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.not-found {
  color: $text1;
  text-align: center;
  padding-top: 220px;
  position: relative;
  background-color: white;
  min-height: calc(100vh - 240px);
}
.not-found-title {
  @mixin title 28px;
}
.not-found-text {
  @mixin title-regular 16px;
  color: $text2;
  margin-top: 24px;
}
</style>
