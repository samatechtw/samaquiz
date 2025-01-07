<template>
  <Modal :show="!!answer" cls="edit-answer-modal" @cancel="emit('cancel')">
    <div class="edit-answer f-center-col">
      <div class="modal-title">
        {{ answer ? ts('answer.edit') : ts('answer.new') }}
      </div>
      <AppInput v-model="text" :label="ts('answer.title')" class="edit-text" />
      <div class="correct-wrap">
        <div class="correct">
          {{ ts('correct') }}
        </div>
        <AppToggle :on="isCorrect" @toggle="isCorrect = $event" />
      </div>
      <ErrorMessage :error="error" />
      <div class="edit-buttons">
        <AppButton
          :text="ts('save')"
          :animate="saving || animate"
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
import {
  AppButton,
  AppInput,
  AppToggle,
  ErrorMessage,
  Modal,
} from '@frontend/components/widgets'
import { IUpdateAnswerApiRequest } from '@frontend/types'
import { apiCreateAnswer, apiUpdateAnswer } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { ts } from '../../i18n'
import { IEditAnswer } from './i-edit-answer'

const { answer } = defineProps<{
  answer: IEditAnswer | undefined
  error?: string
  animate?: boolean
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
  (e: 'confirm'): void
}>()

const text = ref('')
const error = ref()
const isCorrect = ref(false)
const saving = ref(false)

watch(
  () => answer,
  (newAnswer) => {
    text.value = newAnswer?.text ?? ''
    isCorrect.value = !!newAnswer?.is_correct
  },
)

const save = async () => {
  if (!answer) {
    return
  }
  saving.value = true
  try {
    if (answer.id) {
      const payload: IUpdateAnswerApiRequest = { text: text.value }
      if (isCorrect.value !== answer.is_correct) {
        payload.is_correct = isCorrect.value
      }
      await apiUpdateAnswer(answer.id, payload)
    } else {
      await apiCreateAnswer(answer.question_id, {
        text: text.value,
        is_correct: isCorrect.value,
        points: 1,
      })
    }
    emit('confirm')
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  saving.value = false
}
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.edit-answer-modal {
  .edit-answer {
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
  .edit-text {
    margin-top: 24px;
    width: 100%;
  }
  .correct-wrap {
    display: flex;
    margin-top: 16px;
    align-items: center;
  }
  .correct {
    @mixin title 15px;
    color: $text2;
    margin-right: 8px;
  }
}
</style>
