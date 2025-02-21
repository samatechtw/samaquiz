<template>
  <div class="quiz-register">
    <form class="register-wrap" @submit.prevent="participate">
      <div class="register-title">
        {{ ts('session.register') }}
      </div>
      <div class="name-wrap">
        <AppInput v-model="name" :placeholder="ts('name')" class="name" />
        <AppButton :text="ts('session.ready')" type="submit" class="ready" />
      </div>
      <AvatarSelect v-model="avatar" class="avatars" />
      <ErrorMessage :error="error" />
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { apiCreateParticipant } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import {
  AppButton,
  AppInput,
  AvatarSelect,
  ErrorMessage,
} from '@frontend/components/widgets'
import { store } from '@frontend/store'
import { quizSession } from '@frontend/features'
import defaultCat from '@theme/img/cats/cat1.png'
import { ts } from '../../i18n'

const name = ref('')
const avatar = ref(defaultCat)
const error = ref()
const registering = ref(false)

const participate = async () => {
  if (!quizSession.value || registering.value) {
    return
  }
  error.value = undefined
  if (name.value.length < 2) {
    error.value = ts('errors.ParticipantNameShort')
    return
  }
  registering.value = true
  try {
    const { id } = await apiCreateParticipant(quizSession.value.id, {
      name: name.value,
      avatar: avatar.value,
    })
    store.participant.setParticipant(quizSession.value.id, {
      name: name.value,
      avatar: avatar.value,
      participantId: id,
    })
  } catch (e) {
    error.value = ts(errorToKey(e))
  }
  registering.value = false
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.quiz-register {
  padding: 48px 0 80px;
  margin-top: 10%;
}
.register-wrap {
  @mixin quiz-bubble;
}
.register-title {
  @mixin title-regular 20px;
}
.name-wrap {
  display: flex;
}
.name {
  margin: 24px auto 0;
  max-width: 180px;
}
.avatars {
  margin-top: 16px;
}
.ready {
  margin-top: 24px;
}
</style>
