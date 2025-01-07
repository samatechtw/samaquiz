<template>
  <svg height="800" viewBox="-4 0 32 32" width="800" xmlns="http://www.w3.org/2000/svg">
    <path
      :fill="color"
      d="m118 422c-1.657 0-3 1.343-3 3s1.343 3 3 3 3-1.343 3-3-1.343-3-3-3zm0 8c-2.761 0-5-2.238-5-5s2.239-5 5-5 5 2.238 5 5-2.239 5-5 5zm0-17c-6.627 0-12 5.373-12 12 0 5.018 10.005 20.011 12 20 1.964.011 12-15.05 12-20 0-6.627-5.373-12-12-12z"
      fill-rule="evenodd"
      transform="translate(-106 -413)"
    />
  </svg>
</template>

<script lang="ts" setup>
withDefaults(
  defineProps<{
    color?: string
  }>(),
  {
    color: 'rgb(59, 59, 59)',
  },
)
</script>
