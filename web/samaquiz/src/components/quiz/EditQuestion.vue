<template>
  <div class="edit-question-wrap">
    <div v-if="questionState.loading" class="spinner-wrap">
      <Spinner :size="32" />
    </div>
    <div v-else-if="questionState.question" class="edit-question">
      <div class="edit-title">
        <router-link
          :to="{ name: 'EditQuiz', params: { id: questionState.question.quiz_id } }"
          class="back-link"
        >
          <Back class="back" />
        </router-link>
        {{ ts('question.edit') }}
      </div>
      <div class="question-info-wrap">
        <div class="question-left">
          <AppTextArea
            v-model="questionState.question.text"
            :label="ts('text')"
            rows="9"
            class="question-input"
            @update:modelValue="changed = true"
          />
        </div>
        <div class="question-right">
          <AssetUploadArea
            :updating="updating"
            :imageUrl="questionAssetUrl(questionState.question)"
            class="quiz-right"
            @upload="showQuizAssetUpload = true"
            @select="showQuizAssetSelect = true"
          />
        </div>
      </div>
      <div class="save-wrap f-center" :class="{ changed }">
        <AppButton
          :text="ts('save')"
          :animate="updating"
          class="save-button"
          @click="saveQuestion"
        />
      </div>
      <EditQuestionAnswers
        :question="questionState.question"
        :loading="questionState.loading"
        @updated="get"
        @setLoading="questionState.loading = $event"
        @updateAnswerOrder="updateQuestion({ answers_order: $event })"
      />
    </div>
    <NotFound v-else class="no-question" />
    <CreateAssetModal
      v-if="questionState.question"
      :show="showQuizAssetUpload"
      :quizId="questionState.question.quiz_id"
      @complete="questionAssetUploaded"
      @cancel="showQuizAssetUpload = false"
    />
    <SelectAssetModal
      v-if="questionState.question"
      :show="showQuizAssetSelect"
      :quizId="questionState.question.quiz_id"
      :initialUrl="questionState.question.asset_url"
      @select="questionAssetSelected"
      @cancel="showQuizAssetSelect = false"
    />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue'
import { useRoute } from 'vue-router'
import { getQuestion, IGetQuestionParams, IUploadFileResult } from '@frontend/features'
import { errorToKey } from '@frontend/util/api'
import { questionAssetUrl } from '@frontend/util/ui'
import { apiUpdateQuestion } from '@frontend/api'
import { AppButton, AppTextArea, NotFound, Spinner } from '@frontend/components/widgets'
import { Back } from '@frontend/components/svg'
import { ts } from '../../i18n'
import AssetUploadArea from './AssetUploadArea.vue'
import CreateAssetModal from './CreateAssetModal.vue'
import SelectAssetModal from './SelectAssetModal.vue'
import { IUpdateQuestionApiRequest } from '@frontend/types'
import EditQuestionAnswers from './EditQuestionAnswers.vue'

const route = useRoute()

const questionState: IGetQuestionParams = reactive({
  error: undefined,
  loading: false,
  question: undefined,
})
const updateError = ref()
const changed = ref(false)
const updating = ref(false)
const showQuizAssetUpload = ref(false)
const showQuizAssetSelect = ref(false)

const saveQuestion = async () => {
  if (!questionState.question) {
    return
  }
  await updateQuestion({ text: questionState.question.text })
}

const updateQuestion = async (payload: IUpdateQuestionApiRequest) => {
  const id = route.params.id as string
  updateError.value = undefined
  updating.value = true
  try {
    await apiUpdateQuestion(id, payload)
    changed.value = false
    if (questionState.question) {
      Object.assign(questionState.question, {
        ...questionState.question,
        ...payload,
      })
    }
  } catch (e) {
    updateError.value = ts(errorToKey(e))
  }
  updating.value = false
}

const get = () => {
  const id = route.params.id as string
  getQuestion(id, questionState)
}

const questionAssetUploaded = async (asset: IUploadFileResult) => {
  await questionAssetSelected(asset.url)
}

const questionAssetSelected = async (url: string) => {
  const payload: IUpdateQuestionApiRequest = { asset_url: url }
  if (questionState.question && changed.value) {
    payload.text = questionState.question.text
  }
  await updateQuestion(payload)
  showQuizAssetUpload.value = false
  showQuizAssetSelect.value = false
}

onMounted(() => {
  get()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.spinner-wrap {
  text-align: center;
}
.edit-question-wrap {
  max-width: 720px;
  margin: 40px auto 0;
}
.edit-title {
  @mixin title-extra 24px;
  display: flex;
  align-items: center;
}
.back {
  @mixin size 24px;
}
.back-link {
  display: flex;
  margin-right: 12px;
}
.question-info-wrap {
  display: flex;
}
.question-left {
  width: 50%;
}
.question-right {
  width: 50%;
}
.question-input {
  margin-top: 24px;
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
