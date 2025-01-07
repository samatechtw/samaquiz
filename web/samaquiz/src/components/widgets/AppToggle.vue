<template>
  <div class="app-toggle" :class="{ on, small }" @click="emit('toggle', !on)">
    <div class="indicator"></div>
    <div class="text f-center on">
      {{ onText ?? ts('yes') }}
    </div>
    <div class="text f-center off">
      {{ offText ?? ts('no') }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ts } from '../../i18n'

withDefaults(
  defineProps<{
    on: boolean
    onText?: string
    offText?: string
    small?: boolean
  }>(),
  {
    onText: undefined,
    offText: undefined,
  },
)

const emit = defineEmits<{
  (e: 'toggle', on: boolean): void
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.indicator {
  @mixin size 28px;
  position: absolute;
  border-radius: 50%;
  background-color: $color5;
  left: 4px;
  top: 4px;
  transition: left 0.2s ease;
  z-index: 2;
}

.app-toggle {
  @mixin title 14px;
  display: flex;
  position: relative;
  width: 72px;
  height: 36px;
  border-radius: 22px;
  background-color: $color3;
  transition: all 0.2s ease;
  cursor: pointer;
  padding: 0 6px;
  user-select: none;
  &.on {
    background-color: $color3;
    .indicator {
      left: 38px;
    }
    .off {
      opacity: 0;
    }
    .on {
      opacity: 1;
    }
  }
}
.text {
  position: relative;
  color: $color5;
  width: 50%;
  transition: opacity 0.2s ease;
  z-index: 1;
}
</style>
