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
@import '@theme/css/defines.postcss';
</style>
