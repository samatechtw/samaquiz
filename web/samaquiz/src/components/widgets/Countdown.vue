<template>
  <div class="countdown-wrap f-center-col">
    <div v-if="text" class="countdown-text">
      {{ text }}
    </div>
    <div class="countdown">
      {{ countdown }}
    </div>
    <slot />
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'

defineProps<{
  text?: string
}>()
const emit = defineEmits<{
  (e: 'complete'): void
}>()

const countdown = defineModel<number>({ default: -1 })
let countdownInterval: ReturnType<typeof setInterval> | undefined

const startCountdown = () => {
  if (countdown.value > 0 !== undefined && !countdownInterval) {
    countdownInterval = setInterval(() => {
      if (countdown.value > 0) {
        countdown.value -= 1
      } else {
        clearInterval(countdownInterval)
        countdownInterval = undefined
        setTimeout(() => emit('complete'), 1000)
      }
    }, 1000)
  }
}

onMounted(() => {
  startCountdown()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.countdown-wrap {
  @mixin quiz-bubble;
  @mixin title 20px;
  margin-top: 24px;
  width: 240px;
  color: $text2;
}
.countdown-text {
  margin-bottom: 16px;
}
.countdown {
  font-size: 100px;
  color: $text1;
}
</style>
