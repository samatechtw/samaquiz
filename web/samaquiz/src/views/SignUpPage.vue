<template>
  <div class="sign-up-page">
    <div class="content-wrap">
      <SignUp />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { SignUp } from '@frontend/components/auth'
</script>

<style lang="postcss" scoped>
.sign-up-page {
  background-color: white;
}
</style>
