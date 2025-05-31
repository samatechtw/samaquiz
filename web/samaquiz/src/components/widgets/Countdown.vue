<template>
  <div v-if="loading" class="countdown-loading countdown-wrap f-center-col">
    <Spinner :size="20" color="#3282b8" />
  </div>
  <div v-else class="countdown-wrap f-center-col">
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
import { Spinner } from '@frontend/components/widgets'

defineProps<{
  text?: string
  loading?: boolean
}>()
const emit = defineEmits<{
  (e: 'complete'): void
}>()

const countdown = defineModel<number>({ default: -1 })
let countdownInterval: ReturnType<typeof setInterval> | undefined

const startCountdown = () => {
  if (countdown.value > 0 && !countdownInterval) {
    countdownInterval = setInterval(() => {
      if (countdown.value > 0) {
        countdown.value -= 1
      } else {
        clearInterval(countdownInterval)
        countdownInterval = undefined
        setTimeout(() => emit('complete'), 500)
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
.countdown-loading {
  min-height: 174px;
}
.countdown-text {
  margin-bottom: 16px;
}
.countdown {
  font-size: 100px;
  color: $text1;
}
</style>
