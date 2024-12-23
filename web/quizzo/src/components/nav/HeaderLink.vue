<template>
  <div class="header-link">
    <div class="link-bg"></div>
    <router-link :to="{ name: to }" class="link" :class="{ active }">
      {{ text }}
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()

const { to } = defineProps<{
  to: string
  text: string
}>()

const active = computed(() => {
  return route.name === to
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.line {
  background-color: rgb(26, 170, 226);
  height: 1px;
  width: 35px;
  opacity: 0;
  margin-top: 4px;
  transition: opacity 0.25s ease;
}
.link-bg {
  position: absolute;
  top: 0;
  width: 100%;
  height: 100%;
  filter: blur(10px);
  opacity: 0;
  transition: all 0.25s ease;
  background: radial-gradient(
    50% 50% at 50% 50%,
    rgb(255, 255, 255) 0%,
    rgba(255, 255, 255, 0) 100%
  );
}
.header-link {
  @mixin title 16px;
  margin: 0 24px;
  transition: color 0.25s ease;
  align-items: center;
  position: relative;
  &.active {
    pointer-events: none;
    cursor: default;
    .line {
      opacity: 1;
    }
  }
  &:hover .link-bg {
    opacity: 1;
    filter: blur(25px);
  }
}
.link {
  color: white;
  position: relative;
}

@media (max-width: 880px) {
  .header-link {
    margin: 0 12px;
  }
}
</style>
