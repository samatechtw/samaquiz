<template>
  <div class="edit-question-answers f-col">
    <div class="answers-title-wrap">
      <div class="answers-title">
        {{ ts('answers') }}
      </div>
      <AppButton :text="ts('new')" @click="updateAnswerIndex = -1" />
    </div>
    <div class="answers">
      <div
        v-for="(answer, index) in answers"
        class="answer"
        :class="{ dragging: newPos === index }"
        @dragenter="dragenter($event, index)"
        @dragover="dragover"
        @dragleave="dragleave"
        @drag="drag"
        @drop="drop($event, index)"
      >
        <div
          :draggable="true"
          class="drag-wrap f-col"
          @dragstart="dragstart($event, index)"
          @dragend="dragend"
        >
          <DragVertical class="drag" />
        </div>
        <div class="answer-title">
          {{ answer.text }}
        </div>
        <div v-if="answer.is_correct" class="correct">
          {{ ts('correct') }}
        </div>
        <div class="icon-wrap edit-wrap">
          <Edit class="answer-icon" @click="updateAnswerIndex = index" />
        </div>
        <div class="icon-wrap">
          <Trash class="answer-icon" @click="deleteId = answer.id" />
        </div>
      </div>
    </div>
    <EditAnswerModal
      :answer="answerToEdit"
      @confirm="hideAnswerModal"
      @cancel="updateAnswerIndex = undefined"
    />
    <DeleteModal
      :show="!!deleteId"
      :title="ts('answer.delete')"
      :text="ts('answer.delete_text')"
      :animate="loading"
      :error="deleteError"
      @confirm="deleteAnswer"
      @cancel="hideDeleteModal"
    />
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { useListDrag } from '@frontend/util/ui'
import { AppButton } from '@frontend/components/widgets'
import { Edit, Trash, DragVertical } from '@frontend/components/svg'
import { IAnswerViewModel, IGetQuestionApiResponse } from '@frontend/types'
import { ts } from '../../i18n'
import EditAnswerModal from './EditAnswerModal.vue'
import { IEditAnswer } from './i-edit-answer'
import { apiDeleteAnswer } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import DeleteModal from './DeleteModal.vue'
import { sortById } from '../../util/misc/sort-by-id'

const { question } = defineProps<{
  question: IGetQuestionApiResponse
  loading: boolean
}>()

const emit = defineEmits<{
  (e: 'updated'): void
  (e: 'setLoading', loading: boolean): void
  (e: 'updateAnswerOrder', answerIds: string[]): void
}>()

const updateAnswerIndex = ref()
const deleteId = ref<string | undefined>()
const deleteError = ref()

const hideAnswerModal = () => {
  updateAnswerIndex.value = undefined
  emit('updated')
}

const answerToEdit = computed<IEditAnswer | undefined>(() => {
  if (updateAnswerIndex.value === undefined) {
    return
  } else if (question && updateAnswerIndex.value === -1) {
    return {
      question_id: question.id,
      text: '',
      is_correct: false,
      points: 1,
    }
  }

  return question.answers[updateAnswerIndex.value]
})

const hideDeleteModal = () => {
  deleteId.value = undefined
  deleteError.value = undefined
}

const deleteAnswer = async () => {
  if (question && deleteId.value) {
    emit('setLoading', true)
    try {
      await apiDeleteAnswer(deleteId.value)
      hideDeleteModal()
      emit('updated')
    } catch (e) {
      deleteError.value = ts(errorToKey(e))
    }
    emit('setLoading', false)
  }
}

const updateAnswerOrder = () => {
  const ids = answers.value.map((a) => a.id)
  emit('updateAnswerOrder', ids)
}

const {
  newPos,
  dragListPreview,
  dragstart,
  drag,
  dragenter,
  dragover,
  dragleave,
  drop,
  dragend,
} = useListDrag({
  dataIdentifier: 'text/answer-id',
  getDragElement: (e) => (e.target as HTMLElement).parentElement,
  onDrop: (_e, _dragIndex, _targetIndex) => updateAnswerOrder(),
})

const answers = computed(() => {
  const sorted = sortById(question.answers, question.answers_order)
  return dragListPreview(sorted)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.answers-title-wrap {
  display: flex;
  margin-top: 16px;
  align-items: center;
  justify-content: space-between;
}
.answers-title {
  @mixin title 18px;
}
.questions {
  @mixin title-regular 14px;
}
.answer {
  @mixin list-item;
  margin-top: 8px;
  align-items: center;
  padding: 0 20px 0 16px;
  height: 48px;
  border: 1px solid $border1;
  &.dragging {
    background-color: rgba(0, 0, 0, 0.1);
  }
}
.answer-title {
  @mixin title 15px;
}
.correct {
  @mixin title 13px;
  margin: 1px 0 0 20px;
  color: $color3;
}
.icon-wrap {
  display: flex;
  cursor: pointer;
  margin-left: 8px;
}
.edit-wrap {
  margin-left: auto;
}
.answer-icon {
  @mixin size 20px;
  margin-left: 12px;
}
</style>
