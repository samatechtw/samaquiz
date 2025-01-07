<template>
  <div class="quizzes-wrap">
    <div class="quizzes">
      <div class="f-center">
        <AppButton
          :text="ts('quiz.new')"
          class="create-button"
          @click="showCreate = true"
        />
        <SimpleFileUpload class="import" @selectFile="importQuiz">
          {{ ts('import') }}
        </SimpleFileUpload>
      </div>
      <div v-for="quiz in state.quizzes" class="quiz-wrap">
        <router-link :to="{ name: 'EditQuiz', params: { id: quiz.id } }" class="quiz">
          <div class="quiz-title">
            {{ quiz.title }}
          </div>
          <div class="questions">
            {{ `${quiz.questions_order.length} ${ts('questions')}` }}
          </div>
        </router-link>
        <div class="icon-wrap f-center" @click.stop="deleteId = quiz.id">
          <Trash class="quiz-icon" />
        </div>
      </div>
      <div v-if="importing" class="load-overlay f-center overlay">
        <Spinner :size="48" />
      </div>
    </div>
    <CreateQuizModal
      :show="showCreate"
      @complete="quizCreated"
      @cancel="showCreate = false"
    />
    <DeleteModal
      :show="!!deleteId"
      :title="ts('quizzes.delete')"
      :text="ts('quizzes.delete_text')"
      :error="deleteError"
      @confirm="deleteQuiz"
      @cancel="hideDeleteModal"
    />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, reactive, ref } from 'vue'
import { useRouter } from 'vue-router'
import { IListQuizParams, listQuizzes } from '@frontend/features'
import { store } from '@frontend/store'
import {
  apiCreateAnswer,
  apiCreateQuestion,
  apiCreateQuiz,
  apiDeleteQuiz,
  apiUpdateQuestion,
  apiUpdateQuiz,
} from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { Trash } from '@frontend/components/svg'
import { AppButton, SimpleFileUpload, Spinner } from '@frontend/components/widgets'
import { ts } from '../../i18n'
import DeleteModal from './DeleteModal.vue'
import CreateQuizModal from './CreateQuizModal.vue'
import { loadFile } from '@frontend/util/misc'

const router = useRouter()

const state: IListQuizParams = reactive({
  error: undefined,
  loading: false,
  quizzes: [],
})
const deleteId = ref<string | undefined>()
const deleteError = ref()
const showCreate = ref(false)
const importing = ref(false)
const importError = ref()

const hideDeleteModal = () => {
  deleteId.value = undefined
  deleteError.value = undefined
}

const deleteQuiz = async () => {
  if (deleteId.value) {
    state.loading = true
    try {
      await apiDeleteQuiz(deleteId.value)
      hideDeleteModal()
      await list()
    } catch (e) {
      deleteError.value = ts(errorToKey(e))
    }
    state.loading = false
  }
}

const importQuiz = async (file: File) => {
  const quizString = await loadFile(file)
  if (quizString) {
    importing.value = true
    importError.value = undefined
    try {
      const quiz = JSON.parse(quizString)
      const result = await apiCreateQuiz({
        title: quiz.title,
        description: quiz.description,
      })
      const questionsOrder = []
      for (const question of quiz.questions ?? []) {
        const qRes = await apiCreateQuestion(result.id, { text: question.text })
        questionsOrder.push(qRes.id)
        const answersOrder = []
        for (const answer of question.answers ?? []) {
          const aRes = await apiCreateAnswer(qRes.id, {
            text: answer.text,
            is_correct: answer.is_correct,
            points: answer.points,
          })
          answersOrder.push(aRes.id)
        }
        await apiUpdateQuestion(qRes.id, { answers_order: answersOrder })
      }
      await apiUpdateQuiz(result.id, { questions_order: questionsOrder })
      await list()
    } catch (e) {
      importError.value = ts(errorToKey(e))
    }
    importing.value = false
  } else {
    // TODO -- error
  }
}

const quizCreated = (id: string) => {
  router.push({ name: 'EditQuiz', params: { id } })
}

const list = () => {
  return listQuizzes({ user_id: store.auth.userId.value }, state)
}

onMounted(() => {
  list()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quizzes-wrap {
  min-height: 240px;
  position: relative;
}
.quiz-wrap {
  @mixin list-item;
  border: 1px solid $border1;
  &:hover {
    border-color: $color3;
  }
}
.quiz {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 12px 24px;
  cursor: pointer;
  color: $text1;
}
.quiz-title {
  @mixin title 15px;
  @mixin truncate;
  min-width: 160px;
  max-width: 160px;
}
.create-wrap {
  margin-top: 12px;
}
.questions {
  @mixin title-regular 14px;
  margin-left: 8px;
}
.icon-wrap {
  cursor: pointer;
  margin-left: auto;
  padding: 0 16px;
}
.quiz-icon {
  @mixin size 20px;
}
.import {
  @mixin title-regular 16px;
  margin-left: 24px;
  text-decoration: underline;
  user-select: none;
  cursor: pointer;
}
.load-overlay {
  background-color: rgba(0, 0, 0, 0.5);
}
</style>
