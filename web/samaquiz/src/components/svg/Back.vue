<template>
  <svg fill="none" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
    <!-- eslint-disable max-len -->
    <path
      clip-rule="evenodd"
      d="m11.7071 4.29289c.3905.39053.3905 1.02369 0 1.41422l-5.29289 5.29289h13.58579c.5523 0 1 .4477 1 1s-.4477 1-1 1h-13.58579l5.29289 5.2929c.3905.3905.3905 1.0237 0 1.4142s-1.0237.3905-1.4142 0l-7.00001-7c-.18753-.1875-.29289-.4419-.29289-.7071s.10536-.5196.29289-.7071l7.00001-7.00001c.3905-.39052 1.0237-.39052 1.4142 0z"
      :fill="color"
      fill-rule="evenodd"
    />
    <!-- eslint-enable max-len -->
  </svg>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    color?: string
  }>(),
  {
    color: 'rgb(34, 79, 75)',
  },
)
</script>
