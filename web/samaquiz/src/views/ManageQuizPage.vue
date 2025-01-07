<template>
  <div class="manage-quiz">
    <div v-if="quizState.loading" class="spinner-wrap f-center-col">
      <Spinner :size="32" />
    </div>
    <div v-else-if="quizState.quiz" class="quiz-wrap">
      <QuizTop :quizId="quizState.quiz.id" @export="exportQuiz" />
      <RouterTabView :routes="routes" variant="link" class="quiz-router-tabs" />
    </div>
    <NotFound v-else class="no-quiz" />
    <NewSessionModal
      :status="quizState.newSession"
      :animate="creatingSession"
      :error="createError"
      @create="createSession"
      @cancel="quizState.newSession = NewSessionStatus.None"
    />
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { format } from 'date-fns'
import { RouterTabView } from '@frontend/components/nav'
import { QuizTop, NewSessionModal } from '@frontend/components/quiz'
import { NotFound, Spinner } from '@frontend/components/widgets'
import { getQuiz, NewSessionStatus, quizState } from '@frontend/features'
import { apiCreateSession, apiGetQuestion } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { saveFile } from '@frontend/util/misc'
import { IRoute } from '@frontend/types'
import defaultCat from '@theme/img/cats/cat1.png'
import { ts } from '../i18n'

const route = useRoute()

const routes: IRoute[] = [
  {
    name: 'EditQuiz',
    label: ts('edit'),
    labelClass: 'edit-tab',
  },
  {
    name: 'QuizSessions',
    label: ts('sessions'),
    labelClass: 'sessions-tab',
  },
]

const creatingSession = ref(false)
const createError = ref()

const deleteId = ref<string | undefined>()

const createSession = async (code: string) => {
  if (!quizState.quiz) {
    return
  }
  createError.value = undefined
  creatingSession.value = true
  try {
    await apiCreateSession(quizState.quiz.id, {
      code,
      host_avatar: defaultCat,
      host_name: 'Sama',
    })
    await getQuiz(quizState.quiz.id)
    quizState.newSession = NewSessionStatus.Created
  } catch (e) {
    createError.value = ts(errorToKey(e))
  }
  creatingSession.value = false
}

const deleteQuiz = () => {
  if (deleteId.value) {
  }
}

const exportQuiz = async () => {
  if (!quizState.quiz) {
    return
  }
  const { title, description, quiz_type, questions, questions_order } = quizState.quiz

  const date = format(new Date(), 'yyyyMMdd-HHmmss')
  const fileName = `quiz-${title.replaceAll(' ', '-')}${date}.json`
  const result: any = { title, description, quiz_type, questions: [] }

  for (const questionId of questions_order) {
    // Add questions in order
    const q = await apiGetQuestion(questionId)
    const serializedQuestion: any = { text: q.text, answers: [] }
    for (const answerId of q.answers_order) {
      const a = q.answers.find((a) => a.id === answerId)
      if (a) {
        serializedQuestion.answers.push({
          text: a.text,
          is_correct: a.is_correct,
          points: a.points,
        })
      }
    }
    result.questions.push(serializedQuestion)
  }
  saveFile(fileName, JSON.stringify(result, null, 2), 'json')
}

onMounted(() => {
  const id = route.params.id as string
  getQuiz(id)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.manage-quiz {
  padding-top: 140px;
  padding-bottom: 80px;
  background-color: $color5;
}
.quiz-content {
  min-height: calc(100vh - $header-height);
}
.spinner-wrap {
  min-height: calc(100vh - $header-height);
}
.quiz-router-tabs {
  max-width: 720px;
  margin: 40px auto 0;
  :deep(.router-tab-view-tabs) {
    margin-bottom: 16px;
    position: relative;
  }
  &.hide {
    :deep(.router-tab-view-tabs) {
      display: none;
    }
  }
}
@media (max-width: 600px) {
  .quiz-router-tabs {
    padding: 0;
    :deep(.router-tab-view-tabs) {
      justify-content: center;
    }
    :deep(.router-tab-view-tab) {
      &.router-tab-view-tab--link {
        margin-right: 16px;
        font-size: 15px;
      }
    }
  }
}
</style>
