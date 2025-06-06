<template>
  <div class="edit-quiz">
    <div v-if="quizState.quiz" class="questions-wrap">
      <div class="quiz-info-wrap">
        <div class="quiz-left">
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
            rows="7"
            @update:modelValue="changed = true"
          />
        </div>
        <AssetUploadArea
          :updating="updating"
          :imageUrl="quizBackgroundUrl(quizState?.quiz)"
          class="quiz-right"
          @upload="showIntroBackgroundUpload = true"
          @select="showIntroBackgroundSelect = true"
        />
      </div>
      <div class="save-wrap f-center" :class="{ changed }">
        <AppButton
          :text="ts('save')"
          :animate="updating"
          class="save-button"
          @click="saveQuiz"
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
    <CreateAssetModal
      v-if="quizState.quiz"
      :show="showIntroBackgroundUpload"
      :quizId="quizState.quiz.id"
      @complete="introBackgroundUploaded"
      @cancel="showIntroBackgroundUpload = false"
    />
    <SelectAssetModal
      v-if="quizState.quiz"
      :show="showIntroBackgroundSelect"
      :quizId="quizState.quiz.id"
      :initialUrl="quizState.quiz.intro_background_url"
      @select="introBackgroundSelected"
      @cancel="showIntroBackgroundSelect = false"
    />
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { getQuiz, IUploadFileResult, quizState } from '@frontend/features'
import { apiDeleteQuestion, apiUpdateQuiz } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { Edit, Trash } from '@frontend/components/svg'
import { AppButton, AppInput, AppTextArea } from '@frontend/components/widgets'
import DeleteModal from './DeleteModal.vue'
import CreateQuestionModal from './CreateQuestionModal.vue'
import { ts } from '../../i18n'
import CreateAssetModal from './CreateAssetModal.vue'
import { IUpdateQuizApiRequest } from '@frontend/types'
import SelectAssetModal from './SelectAssetModal.vue'
import { quizBackgroundUrl } from '@frontend/util/ui'
import AssetUploadArea from './AssetUploadArea.vue'

const route = useRoute()
const router = useRouter()

const deleteId = ref<string | undefined>()
const deleteError = ref()
const updateError = ref()
const changed = ref(false)
const updating = ref(false)
const showCreate = ref(false)
const showIntroBackgroundUpload = ref(false)
const showIntroBackgroundSelect = ref(false)

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

const saveQuiz = async () => {
  if (!quizState.quiz) {
    return
  }
  await updateQuiz({
    title: quizState.quiz.title,
    description: quizState.quiz.description,
  })
}

const updateQuiz = async (payload: IUpdateQuizApiRequest) => {
  const id = route.params.id as string
  updateError.value = undefined
  updating.value = true
  try {
    await apiUpdateQuiz(id, payload)
    if (quizState.quiz) {
      Object.assign(quizState.quiz, {
        ...quizState.quiz,
        ...payload,
      })
    }
    changed.value = false
  } catch (e) {
    updateError.value = ts(errorToKey(e))
  }
  updating.value = false
}

const introBackgroundUploaded = async (asset: IUploadFileResult) => {
  await introBackgroundSelected(asset.url)
}

const introBackgroundSelected = async (url: string) => {
  const payload: IUpdateQuizApiRequest = { intro_background_url: url }
  if (quizState.quiz && changed.value) {
    payload.title = quizState.quiz.title
    payload.description = quizState.quiz.description
  }
  await updateQuiz(payload)
  showIntroBackgroundUpload.value = false
  showIntroBackgroundSelect.value = false
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
.quiz-info-wrap {
  display: flex;
}
.quiz-left {
  width: 50%;
}
.quiz-right {
  width: 50%;
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
