<template>
  <div ref="wrapper" class="spinner-wrap" :class="{ 'spinner-wrap--max': max }">
    <span class="dot" />
    <span class="dot" />
    <span class="dot" />
  </div>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    size?: number
    color?: string
    max?: boolean
  }>(),
  {
    size: 10,
    color: '#fff',
  },
)
</script>

<style lang="postcss" scoped>
.spinner-wrap {
  display: inline-flex;
  align-items: center;
  &.spinner-wrap--max {
    display: flex;
    justify-content: center;
    width: 100%;
    height: 100%;
  }
}
.dot {
  width: v-bind(size + 'px');
  height: v-bind((size * 0.7) + 'px');
  background-color: v-bind(color);
  animation: scale 0.6s ease alternate infinite;
  &:not(:first-child) {
    margin-left: calc(v-bind(size + 'px') * 0.6);
  }
  &:nth-of-type(2) {
    animation-delay: 0.2s;
  }
  &:nth-of-type(3) {
    animation-delay: 0.4s;
  }
}
@keyframes scale {
  0% {
    transform: scale(0);
  }
  100% {
    transform: scale(1);
  }
}
</style>
