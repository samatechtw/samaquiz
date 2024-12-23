<template>
  <div class="app-textarea-wrap">
    <textarea
      ref="inputRef"
      :class="['app-textarea', inputClass]"
      :value="modelValue"
      :type="type"
      :disabled="isDisabled"
      :placeholder="placeholder"
      :name="inputName"
      :rows="rows"
      @input="handleInput"
      @focusout="emit('focusout', modelValue)"
      @keyup="emit('keyup', $event)"
      @keydown="emit('keydown', $event)"
      @keyup.enter="emit('handle-enter')"
    />
    <label
      v-if="label"
      class="app-textarea-label"
      :for="inputName"
      :class="{ active: showTitleTop }"
    >
      <span>{{ label }}</span>
    </label>
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed, toRefs, useId } from 'vue'

const props = withDefaults(
  defineProps<{
    modelValue?: number | string
    placeholder?: string
    label?: string | null
    type?: string
    name?: string
    rows?: string
    isDisabled?: boolean
    inputClass?: string | null
    maxLength?: number
  }>(),
  {
    modelValue: '',
    placeholder: '',
    label: null,
    type: 'text',
    rows: '5',
    name: undefined,
    inputClass: null,
    maxLength: undefined,
  },
)
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'focusout', value: string | number | undefined): void
  (e: 'keyup', event: KeyboardEvent): void
  (e: 'keydown', event: KeyboardEvent): void
  (e: 'handle-enter'): void
  (e: 'clear'): void
}>()

const { type, name, placeholder, modelValue } = toRefs(props)

const uid = useId()

const inputName = computed(() => {
  return name.value || `input${uid}`
})

const showTitleTop = computed(() => {
  return placeholder.value || modelValue.value || modelValue.value === 0
})

const handleInput = (e: Event) => {
  const value = (e.target as HTMLInputElement)?.value
  emit('update:modelValue', value)
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.app-textarea-wrap {
  position: relative;
  display: flex;
  align-items: flex-start;
  flex-wrap: wrap;
}

.app-textarea-label {
  @mixin title-thin 14px;
  position: absolute;
  top: 10px;
  padding: 0 4px;
  margin-left: 8px;
  color: rgba(0, 0, 0, 0.6);
  transition: ease 0.2s all;
  pointer-events: none;
}

.app-textarea-label.active,
.app-textarea:focus ~ .app-textarea-label {
  @mixin title-thin 11px;
  top: -9px;
  background-color: white;
  border-top-left-radius: 4px;
  border-top-right-radius: 4px;
  color: rgba(0, 0, 0, 0.9);
  padding-top: 2px;
}
.app-textarea {
  @mixin title-regular 13px;
  @mixin input;
  box-sizing: border-box;
  color: $text1;
  width: 100%;
  border-radius: 2px;
  border: 1px solid $border1;
  outline: none;
  outline-style: none;
  box-shadow: none;
  padding: 8px 10px;
  transition:
    border-color 0.3s,
    background-color 0.3s;

  &::placeholder {
    color: $text1;
    opacity: 0.5;
  }

  &:focus {
    border-color: $primary;
    background-color: white;
    &::-webkit-input-placeholder {
      opacity: 0;
      transition: opacity 0.2s ease;
    }
  }

  &:disabled {
    border: 1px solid $border1;
    color: $disabled1;
    -webkit-text-fill-color: $disabled1;
    background-color: $bg-light1;
  }

  /* stylelint-disable */
  &:-internal-autofill-selected,
  &:-webkit-autofill,
  &:-webkit-autofill:hover,
  &:-webkit-autofill:focus,
  &:-webkit-autofill:active {
    /* Disable background color of input autocomplete */
    box-shadow: 0 0 0 100px #fff inset !important;
    font-size: initial !important;
  }
  /* stylelint-enable */

  &[type='number'] {
    appearance: textfield;
    -moz-appearance: textfield;
  }
  &[type='number']::-webkit-inner-spin-button,
  &[type='number']::-webkit-outer-spin-button {
    appearance: none;
    -webkit-appearance: none;
    margin: 0;
  }
}
</style>
