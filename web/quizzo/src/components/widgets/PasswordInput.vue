<template>
  <AppInput
    :modelValue="modelValue"
    :type="inputType"
    :placeholder="placeholder"
    :name="name"
    :errorBubble="errorMessage"
    :isDisabled="isDisabled"
    :inputClass="inputClass"
    :maxLength="maxLength"
    class="has-icon-button"
    @update:modelValue="emit('update:modelValue', $event)"
    @focusout="emit('focusout', $event)"
    @handle-enter="emit('handle-enter')"
  >
    <IconButtonWrap
      class="eye-button-wrap"
      :iconSize="18"
      :buttonSize="18"
      :disabled="isDisabled"
      @click="showPasswordFlag = !showPasswordFlag"
    >
      <Show v-if="showPasswordFlag" />
      <Hide v-else />
    </IconButtonWrap>
  </AppInput>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { Show, Hide } from '@frontend/components/svg'
import AppInput from './AppInput.vue'
import IconButtonWrap from './IconButtonWrap.vue'

withDefaults(
  defineProps<{
    modelValue?: number | string
    placeholder?: string
    name?: string
    errorMessage?: string | undefined
    isDisabled?: boolean
    inputClass?: string | null
    maxLength?: number
  }>(),
  {
    modelValue: '',
    placeholder: '',
    name: undefined,
    errorMessage: undefined,
    isDisabled: false,
    inputClass: null,
    maxLength: undefined,
  },
)
const emit = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'focusout', value: string | number | undefined): void
  (e: 'handle-enter'): void
}>()

const showPasswordFlag = ref(false)

const inputType = computed(() => {
  return showPasswordFlag.value ? 'text' : 'password'
})
</script>
