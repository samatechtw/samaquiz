<template>
  <Modal :show="show" cls="delete-modal" @cancel="emit('cancel')">
    <div class="delete f-center-col">
      <div class="modal-title">
        {{ title }}
      </div>
      <div class="modal-text">
        {{ text }}
      </div>
      <ErrorMessage :error="error" />
      <div class="delete-buttons">
        <AppButton
          :text="ts('delete')"
          :animate="animate"
          class="delete-button button2"
          @click="emit('confirm')"
        />
        <AppButton
          :text="ts('cancel')"
          class="button2 cancel-button"
          @click="emit('cancel')"
        />
      </div>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { AppButton, ErrorMessage, Modal } from '@frontend/components/widgets'
import { ts } from '../../i18n'

const { show } = defineProps<{
  show: boolean
  title: string
  text: string
  error?: string
  animate?: boolean
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
  (e: 'confirm'): void
}>()
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.delete-modal {
  .delete-button {
    width: 110px;
    margin: 0 24px 0 0;
  }
  .cancel-button {
    width: 110px;
  }
  .delete-buttons {
    margin-top: 24px;
    display: flex;
  }
}
</style>
