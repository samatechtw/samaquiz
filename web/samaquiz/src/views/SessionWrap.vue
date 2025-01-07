<template>
  <SessionHeader />
  <div id="scroll-container" class="session-content">
    <router-view class="app-router-view" />
  </div>
</template>

<script lang="ts" setup>
import { SessionHeader } from '@frontend/components/nav'
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

#app {
  height: 100%;
}

.session-content {
  background-color: $color5;
  min-height: 100%;
  position: relative;
}
</style>
