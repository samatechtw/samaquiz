<template>
  <div class="login-page">
    <div class="content-wrap">
      <Login />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Login } from '@frontend/components/auth'
</script>

<style lang="postcss" scoped>
.login-page {
  background-color: white;
}
</style>
