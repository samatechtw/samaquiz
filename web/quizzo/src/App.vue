<template>
  <AppHeader />
  <div id="scroll-container" class="app-content">
    <router-view class="app-router-view" />
    <AppFooter />
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'
import { AppHeader, AppFooter } from '@frontend/components/nav'
import { getUser, loggedIn, useLoginRedirect } from '@frontend/features'

const { watchAuthRedirect } = useLoginRedirect()

watchAuthRedirect()

onMounted(async () => {
  if (loggedIn.value) {
    await getUser()
  }
})
</script>

<style lang="postcss">
@import '@theme/css/font.postcss';
@import '@theme/css/defines.postcss';
@import '@theme/css/app.postcss';
@import '@theme/css/auth.postcss';

html,
body {
  padding: 0;
  margin: 0;
  width: 100%;
  height: 100%;
  color: black;
  background: $color5;
  * {
    box-sizing: border-box;
  }
}
body {
  background: $color1;
}

#app {
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  position: relative;
  overflow: hidden;
}
</style>
