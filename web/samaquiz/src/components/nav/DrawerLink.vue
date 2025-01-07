<template>
  <router-link :to="{ name: to }" class="drawer-link">
    {{ text }}
  </router-link>
</template>

<script lang="ts" setup>
defineProps<{
  to: string
  text: string
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.drawer-link {
  @mixin text 20px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.5);
  padding: 12px 0;
  max-width: 530px;
  margin: 0 auto;
  color: black;
  width: 100%;
  text-align: center;
}
</style>
