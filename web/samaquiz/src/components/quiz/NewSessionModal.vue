<template>
  <Modal
    :show="status !== NewSessionStatus.None"
    cls="new-session-modal"
    @cancel="emit('cancel')"
  >
    <div class="delete f-center-col">
      <div class="modal-title">
        {{ ts('session.session') }}
      </div>
      <div class="modal-text">
        {{ copy.text }}
      </div>
      <div class="code-wrap">
        <AppInput
          v-model="code"
          :disabled="status === NewSessionStatus.Created"
          :label="ts('code')"
        />
      </div>
      <ErrorMessage :error="error" />
      <div class="new-session-buttons">
        <router-link
          v-if="status === NewSessionStatus.Created"
          target="_blank"
          :to="{ name: 'QuizSession', params: { code } }"
        >
          <AppButton
            :text="copy.button"
            :animate="animate"
            class="new-session-button button2"
          />
        </router-link>
        <AppButton
          v-else
          :text="copy.button"
          :animate="animate"
          class="new-session-button button2"
          @click="emit('create', code)"
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
import { ref, watch, computed } from 'vue'
import { AppButton, AppInput, ErrorMessage, Modal } from '@frontend/components/widgets'
import { ts } from '../../i18n'
import { NewSessionStatus } from '@frontend/features'

const { status } = defineProps<{
  status: NewSessionStatus
  animate: boolean
  error?: string
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
  (e: 'create', code: string): void
}>()

const code = ref('')

const copy = computed(() => {
  return status === NewSessionStatus.Created
    ? { button: ts('view'), text: ts('session.view_text') }
    : { button: ts('create'), text: ts('session.new_text') }
})

watch(
  () => status,
  (newStatus) => {
    if (newStatus !== NewSessionStatus.Created) {
      code.value = ''
    }
  },
)
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.new-session-modal {
  .new-session-button {
    width: 110px;
    margin: 0 24px 0 0;
  }
  .cancel-button {
    width: 110px;
  }
  .new-session-buttons {
    margin-top: 24px;
    display: flex;
  }
  .code-wrap {
    width: 70%;
    margin-top: 24px;
  }
}
</style>
