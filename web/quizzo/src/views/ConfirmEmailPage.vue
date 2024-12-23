<template>
  <div class="confirm-email-page">
    <div class="content-wrap">
      <ConfirmEmail />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ConfirmEmail } from '@frontend/components/auth'
</script>

<style lang="postcss" scoped>
.confirm-email-page {
  background-color: white;
}
</style>
