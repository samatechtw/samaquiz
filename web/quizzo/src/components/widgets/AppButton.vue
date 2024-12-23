<template>
  <button
    :class="['button1', disabled && 'disabled']"
    :disabled="disabled"
    @click="click"
  >
    <div v-if="animate" class="button-animate">
      <Spinner class="button-spinner" />
    </div>
    <div v-else class="button-text">
      <span>
        {{ text }}
      </span>
      <slot />
    </div>
    <div v-if="animate" class="hidden-text">
      {{ text }}
    </div>
  </button>
</template>

<script lang="ts" setup>
import Spinner from './Spinner.vue'

const emit = defineEmits(['click'])

const {
  disabled = false,
  animate = false,
  text = '',
} = defineProps<{
  text?: string
  disabled?: boolean
  animate?: boolean
}>()

const click = (e: Event) => {
  if (!disabled) {
    emit('click', e)
  }
}
</script>

<style lang="postcss" scoped>
.hidden-text {
  visibility: hidden;
}

button {
  position: relative;
}

.button-animate {
  position: absolute;
  display: flex;
  justify-content: center;
  width: 100%;
  height: 100%;
  top: 0;
  left: 0;
}
</style>
