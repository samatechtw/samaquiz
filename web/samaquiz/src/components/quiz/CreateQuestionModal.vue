<template>
  <Modal :show="show" cls="create-question-modal" @cancel="emit('cancel')">
    <div class="create-question f-center-col">
      <div class="modal-title">
        {{ ts('question.new') }}
      </div>
      <div class="modal-text">
        {{ ts('question.new_text') }}
      </div>
      <AppInput v-model="text" :label="ts('text')" class="quiz-input" />
      <ErrorMessage :error="error" />
      <div class="edit-buttons">
        <AppButton
          :text="ts('save')"
          :animate="saving"
          class="save-button button2"
          @click="save"
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
import { ref, watch } from 'vue'
import { AppButton, AppInput, ErrorMessage, Modal } from '@frontend/components/widgets'
import { apiCreateQuestion } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { ts } from '../../i18n'

const { show, quizId } = defineProps<{
  show: boolean
  quizId: string
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
  (e: 'complete', id: string): void
}>()

const text = ref('')
const error = ref()
const saving = ref(false)

watch(
  () => show,
  () => {
    text.value = ''
  },
)

const save = async () => {
  saving.value = true
  error.value = undefined
  try {
    const result = await apiCreateQuestion(quizId, {
      text: text.value,
    })
    emit('complete', result.id)
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  saving.value = false
}
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.create-question-modal {
  .create-question {
    margin: 0 auto;
    max-width: 300px;
  }
  .save-button {
    width: 110px;
    margin: 0 24px 0 0;
  }
  .cancel-button {
    width: 110px;
  }
  .edit-buttons {
    margin-top: 24px;
    display: flex;
  }
  .quiz-input {
    width: 100%;
    margin-top: 12px;
  }
  .edit-text {
    margin-top: 24px;
    width: 100%;
  }
}
</style>
