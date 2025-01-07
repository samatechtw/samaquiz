<template>
  <span
    class="ibw f-center"
    :class="{ 'ibw--disabled': disabled }"
    @click="!disabled && emit('click')"
  >
    <slot />
  </span>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    buttonSize?: number
    iconSize?: number
    disabled?: boolean
  }>(),
  {
    buttonSize: 16,
    iconSize: 8,
  },
)

const emit = defineEmits<{
  (e: 'click'): void
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.ibw {
  width: v-bind(buttonSize + 'px');
  height: v-bind(buttonSize + 'px');
  cursor: pointer;
  border-radius: 50%;
  transition: background-color 0.2s;

  :slotted(svg) {
    width: v-bind(iconSize + 'px');
    height: v-bind(iconSize + 'px');
  }

  &:not(.ibw--disabled) {
    &:hover {
      background-color: $border1;
    }
    &:active {
      background-color: $text2;
    }
  }

  &.ibw--disabled {
    cursor: default;
    :slotted(svg) {
      fill: $disabled1;
    }
  }
}
</style>
