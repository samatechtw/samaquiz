<template>
  <div class="ps-forgot-password auth-wrap">
    <div class="auth-title">
      {{ ts('auth.forgot_password') }}
    </div>
    <template v-if="!sent">
      <div class="auth-subtitle">
        {{ ts('auth.forgot_password_subtitle') }}
      </div>
      <form class="auth-form" @submit.prevent="submit">
        <AppInput
          v-model="email"
          type="email"
          name="email"
          class="email-input"
          :placeholder="ts('email')"
          :required="true"
        />
        <ErrorMessage :error="error" />
        <AppButton
          type="submit"
          class="submit-button"
          :text="ts('send')"
          :animate="loading"
        />
      </form>
    </template>
    <div v-else class="auth-subtitle">
      {{ ts('auth.forgot_password_sent') }}
    </div>
    <I18nT keypath="auth.back_to" tag="div" class="back-to-login">
      <router-link :to="{ name: 'Login' }" class="login-link">
        {{ ts('auth.login') }}
      </router-link>
    </I18nT>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import { AppInput, ErrorMessage, AppButton } from '@frontend/components/widgets'
import { apiResetPassword } from '@frontend/api'
import { errorToKey } from '@frontend/util/api'
import { ts } from '../../i18n'

const email = ref('')
const loading = ref(false)
const error = ref('')
const sent = ref(false)

const submit = async () => {
  loading.value = true
  error.value = ''
  try {
    await apiResetPassword(email.value)
    sent.value = true
  } catch (e) {
    error.value = errorToKey(e)
    loading.value = false
  }
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.email-input {
  margin-top: 40px;
}
.back-to-login {
  @mixin title 13px;
  margin-top: 16px;
  color: $text1;
  text-align: center;
  .login-link {
    color: $primary;
  }
}
</style>
