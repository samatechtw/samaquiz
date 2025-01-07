<template>
  <div class="edit-quiz">
    <div v-if="quizState.quiz" class="questions-wrap">
      <AppInput
        v-model="quizState.quiz.title"
        :label="ts('title')"
        class="quiz-input"
        @update:modelValue="changed = true"
      />
      <AppTextArea
        v-model="quizState.quiz.description"
        :label="ts('description')"
        class="quiz-input description"
        @update:modelValue="changed = true"
      />
      <div class="save-wrap f-center" :class="{ changed }">
        <AppButton
          :text="ts('save')"
          :animate="updating"
          class="save-button"
          @click="updateQuiz"
        />
      </div>
      <div class="questions-title">
        {{ ts('questions') }}
      </div>
      <div class="f-center">
        <AppButton
          :text="ts('question.new')"
          class="create-button"
          @click="showCreate = true"
        />
      </div>
      <div class="questions">
        <div v-for="question in quizState.quiz.questions" class="question">
          <div class="question-title">
            {{ question.text }}
          </div>
          <div class="answers">
            {{ `${question.answers_order.length} ${ts('answers')}` }}
          </div>
          <router-link
            :to="{ name: 'EditQuestion', params: { id: question.id } }"
            class="icon-wrap edit-wrap"
          >
            <Edit class="question-icon" />
          </router-link>
          <div class="icon-wrap">
            <Trash class="question-icon" @click="deleteId = question.id" />
          </div>
        </div>
      </div>
      <CreateQuestionModal
        :show="showCreate"
        :quizId="quizState.quiz.id"
        @complete="questionCreated"
        @cancel="showCreate = false"
      />
      <DeleteModal
        :show="!!deleteId"
        :title="ts('question.delete')"
        :text="ts('question.delete_text')"
        :error="deleteError"
        :animate="quizState.loading"
        @confirm="deleteQuestion"
        @cancel="hideDeleteModal"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getQuiz, quizState } from '@frontend/features'
import { apiDeleteQuestion, apiUpdateQuiz } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { Edit, Trash } from '@frontend/components/svg'
import { AppButton, AppInput, AppTextArea } from '@frontend/components/widgets'
import DeleteModal from './DeleteModal.vue'
import CreateQuestionModal from './CreateQuestionModal.vue'
import { ts } from '../../i18n'

const route = useRoute()
const router = useRouter()

const deleteId = ref<string | undefined>()
const deleteError = ref()
const updateError = ref()
const changed = ref(false)
const updating = ref(false)
const showCreate = ref(false)

const hideDeleteModal = () => {
  deleteId.value = undefined
  deleteError.value = undefined
}

const deleteQuestion = async () => {
  if (quizState.quiz && deleteId.value) {
    quizState.loading = true
    try {
      await apiDeleteQuestion(deleteId.value)
      hideDeleteModal()
      await list()
    } catch (e) {
      deleteError.value = ts(errorToKey(e))
    }
    quizState.loading = false
  }
}

const updateQuiz = async () => {
  const id = route.params.id as string
  if (!quizState.quiz) {
    return
  }
  updateError.value = undefined
  updating.value = true
  try {
    await apiUpdateQuiz(id, {
      title: quizState.quiz.title,
      description: quizState.quiz.description,
    })
    changed.value = false
  } catch (e) {
    updateError.value = ts(errorToKey(e))
  }
  updating.value = false
}

const questionCreated = (id: string) => {
  router.push({ name: 'EditQuestion', params: { id } })
}

const list = () => {
  const id = route.params.id as string
  return getQuiz(id)
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.edit-quiz {
  min-height: calc(100vh - 380px);
}

.quizzes-wrap {
  min-height: 240px;
}
.description {
  margin-top: 16px;
}
.questions-title {
  @mixin title 18px;
  border-bottom: 1px solid $border1;
  margin-top: 16px;
}
.create-button {
  margin-top: 12px;
}
.question {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 12px 20px;
  border: 1px solid $border1;
  margin-top: 12px;
  border-radius: 4px;
}
.question-title {
  @mixin title 15px;
  @mixin truncate;
  min-width: 160px;
  max-width: 160px;
}
.questions {
  @mixin title-regular 14px;
}
.answers {
  margin-left: 12px;
  color: $text2;
}
.icon-wrap {
  display: flex;
  cursor: pointer;
  margin-left: 8px;
}
.edit-wrap {
  margin-left: auto;
}
.question-icon {
  @mixin size 20px;
  margin-left: 12px;
}
.save-wrap {
  height: 0;
  opacity: 0;
  transition: all 0.25s ease;
  margin-top: 12px;
  &.changed {
    height: 40px;
    opacity: 1;
  }
  .save-button {
    height: 100%;
  }
}
</style>
