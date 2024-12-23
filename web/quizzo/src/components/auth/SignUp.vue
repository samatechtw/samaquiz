<template>
  <div class="sign-up auth-wrap">
    <div class="auth-title">
      {{ ts('auth.sign_up') }}
    </div>
    <div class="auth-subtitle">
      {{ ts('auth.sign_up_subtitle') }}
    </div>
    <div class="auth-subtitle have-account">
      {{ ts('auth.have_account') }}
      <span class="login-link">
        <router-link :to="{ name: 'Login' }">
          {{ ts('auth.login_here') }}
        </router-link>
      </span>
    </div>
    <form class="auth-form" @submit.prevent="register">
      <AppInput
        v-model="payload.email"
        type="email"
        class="email-input"
        :placeholder="ts('email')"
        :isDisabled="loading"
      />
      <PasswordInput
        v-model="payload.password"
        class="password-input"
        :placeholder="ts('auth.password')"
        :isDisabled="loading"
      />
      <div class="terms-checkbox-wrap">
        <Checkbox :item="termsCheckItem" :disabled="loading" />
        <label
          class="terms-text"
          @click="termsCheckItem.checked = !termsCheckItem.checked"
        >
          {{ ts('auth.terms_agree') }}
          <router-link :to="{ name: 'Terms' }" target="_blank" class="terms-link">
            {{ ts('footer.terms') }}
          </router-link>
        </label>
      </div>
      <ErrorMessage :error="error" />
      <AppButton
        type="submit"
        class="sign-up-button submit-button"
        :text="ts('auth.sign_up')"
        :animate="loading"
        :secondary="true"
        :disabled="!termsCheckItem.checked || loading"
      />
    </form>
  </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue'
import {
  AppInput,
  Checkbox,
  ErrorMessage,
  AppButton,
  PasswordInput,
} from '@frontend/components/widgets'
import { ts } from '../../i18n'
import { errorToKey } from '@frontend/util/api'
import { loginUser, registerUser, useLoginRedirect } from '@frontend/features'
import { IRegisterUserApiRequest } from '@frontend/types'

const { redirectAfterLogin } = useLoginRedirect()

const payload = ref<IRegisterUserApiRequest>({
  email: '',
  password: '',
})
const loading = ref(false)
const error = ref('')

const termsCheckItem = ref({
  label: '',
  checked: false,
})

const validate = (): boolean => {
  const password = payload.value.password
  if (password.length < 8 || password.length > 50) {
    error.value = ts('errors.password_length')
    return false
  }
  return true
}

const register = async () => {
  if (!validate()) {
    return
  }
  loading.value = true
  error.value = ''
  try {
    await registerUser(payload.value)
    await loginUser({ email: payload.value.email, password: payload.value.password })
    await redirectAfterLogin()
    loading.value = false
  } catch (e) {
    error.value = ts(errorToKey(e))
    loading.value = false
  }
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.terms-text {
  @mixin title-thin 13px;
}
.email-input {
  margin-top: 40px;
}
.have-account {
  color: $color3;
  font-size: 15px;
}
.login-link {
  text-decoration: underline;
}
.terms-checkbox-wrap {
  display: flex;
  margin-top: 24px;
  align-items: center;
  color: $text1;
  .checkbox {
    margin: 0;
  }
  .terms-link {
    text-decoration: underline;
  }
}
</style>
