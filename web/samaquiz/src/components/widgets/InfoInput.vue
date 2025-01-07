<template>
  <AppInput
    ref="input"
    :modelValue="modelValue"
    :placeholder="placeholder"
    :label="label"
    :type="type"
    :name="name"
    :autocomplete="autocomplete"
    :isDisabled="isDisabled"
    :inputClass="inputClass"
    :maxLength="maxLength"
    :isError="!!errorBubble"
    :class="cls"
    @update:modelValue="emit('update:modelValue', $event)"
    @focusout="emit('focusout', $event)"
    @handle-enter="emit('handle-enter')"
  >
    <InfoBubble
      v-if="errorBubble || infoBubble"
      :customIcon="true"
      :message="errorBubble || infoBubble || ''"
      placement="top"
      class="input-suffix bubble"
    >
      <Info :color="errorBubble ? '#f46a6a' : '#5d99b6'" class="error-icon" />
    </InfoBubble>
    <slot />
  </AppInput>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { AppInput } from '@frontend/components/widgets'
import { Info } from '@frontend/components/svg'
import InfoBubble from './InfoBubble.vue'

withDefaults(
  defineProps<{
    modelValue?: number | string
    placeholder?: string
    label?: string | null
    type?: string
    name?: string
    autocomplete?: string
    errorBubble?: string | undefined
    infoBubble?: string | undefined
    isDisabled?: boolean
    inputClass?: string | null
    maxLength?: number
    cls?: string | undefined
  }>(),
  {
    modelValue: '',
    placeholder: '',
    label: null,
    type: 'text',
    name: undefined,
    autocomplete: 'off',
    errorBubble: undefined,
    infoBubble: undefined,
    isDisabled: false,
    inputClass: null,
    maxLength: undefined,
    cls: undefined,
  },
)
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'focusout', value: string | number | undefined): void
  (e: 'keyup', event: KeyboardEvent): void
  (e: 'keydown', event: KeyboardEvent): void
  (e: 'handle-enter'): void
}>()

const input = ref()

const focus = () => {
  input.value?.inputRef?.focus()
}

defineExpose({ focus })
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

:deep(.info-bubble-wrap) {
  @mixin text 14px;
  @mixin size 18px;
  color: $text1;
  position: absolute;
  top: 10px;
  right: 12px;
  &:hover {
    svg g {
      fill: rgba($red, 0.7);
    }
  }
}
</style>
