<template>
  <div class="drawer-wrap" :class="{ active }">
    <div class="drawer overlay f-col">
      <div class="drawer-top">
        <router-link :to="{ name: 'Home' }" @click="emit('close')">
          <Logo class="header-logo" />
        </router-link>
        <div class="drawer-close" @click="emit('close')">
          <svg
            width="32"
            height="32"
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 256 256"
          >
            <path
              d="M205.66,194.34a8,8,0,0,1-11.32,11.32L128,139.31,61.66,205.66a8,8,0,0,1-11.32-11.32L116.69,128,50.34,61.66A8,8,0,0,1,61.66,50.34L128,116.69l66.34-66.35a8,8,0,0,1,11.32,11.32L139.31,128Z"
            ></path>
          </svg>
        </div>
      </div>
      <div class="drawer-content f-col">
        <slot />
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { Logo } from '@frontend/components/svg'

defineProps<{
  active: boolean
}>()

const emit = defineEmits<{
  (e: 'close'): void
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.drawer-wrap {
  background-color: $primary;
  height: 480px;
  width: 100%;
  position: absolute;
  top: 0;
  left: 100%;
  transition: left 0.3s ease;
  &.active {
    left: 0;
  }
}
.drawer {
  z-index: 1000;
  height: 100vh;
  backdrop-filter: blur(24px);
  background-color: rgba(255, 255, 255, 0.7);
  overflow: hidden;
}
.header-logo {
  width: 104px;
}
.drawer-top {
  padding: 28px 32px 0;
  display: flex;
  justify-content: space-between;
}
.drawer-content {
  padding: 32px 24px 0;
}
.drawer-close {
  padding: 4px;
  svg {
    fill: black;
  }
}
</style>
