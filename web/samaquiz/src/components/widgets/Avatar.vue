<template>
  <div class="avatar f-center">
    <img :src="url || defaultAvatar" />
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue'
import defaultAvatar from '@theme/img/cats/cat1.png'

const { size } = defineProps<{
  url: string | null | undefined
  size: number | string
}>()

const cssSize = computed(() => {
  return size ? `${size}px` : '100%'
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.avatar {
  width: v-bind(cssSize);
  height: v-bind(cssSize);
  overflow: hidden;
  border-radius: 50%;
  background-color: $color4;
  position: relative;
}
img {
  width: 90%;
  height: 90%;
  position: relative;
  top: 3%;
}
</style>
