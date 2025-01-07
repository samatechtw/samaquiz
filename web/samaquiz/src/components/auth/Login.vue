<template>
  <div class="login auth-wrap">
    <div class="auth-title">
      {{ ts('auth.login') }}
    </div>
    <div v-if="!disableSignup" class="auth-subtitle">
      {{ ts('auth.no_account') }}<br />
      <router-link :to="{ name: 'SignUp' }">
        {{ ts('auth.click_here') }}
      </router-link>
    </div>
    <form class="auth-form" @submit.prevent="login">
      <AppInput
        v-model="payload.email"
        name="email"
        class="email-input"
        type="email"
        :placeholder="ts('email')"
        :isDisabled="loading"
      />
      <PasswordInput
        v-model="payload.password"
        class="password-input"
        name="password"
        :placeholder="ts('auth.password')"
        :isDisabled="loading"
      />
      <ErrorMessage :error="ts(authErrorKey)" />
      <div class="forgot">
        <router-link
          v-if="!disableForgot"
          :to="{ name: 'ForgotPassword' }"
          class="forgot-password-link"
        >
          {{ ts('auth.forgot_password') }}
        </router-link>
      </div>
      <AppButton
        type="submit"
        class="login-button submit-button"
        :text="ts('auth.login')"
        :secondary="true"
        :animate="loading"
        :disabled="loading"
      >
        <template #trailing-icon>
          <ArrowShort />
        </template>
      </AppButton>
    </form>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref, toRefs } from 'vue'
import {
  AppInput,
  ErrorMessage,
  AppButton,
  PasswordInput,
} from '@frontend/components/widgets'
import { ArrowShort } from '@frontend/components/svg'
import { authErrorKey, loginUser, useLoginRedirect } from '@frontend/features'
import { store } from '@frontend/store'
import { errorToKey } from '@frontend/util/api'
import { ILoginUserApiRequest } from '@frontend/types'
import { ts } from '../../i18n'

const props = withDefaults(
  defineProps<{
    disableSignup?: boolean
    disableForgot?: boolean
    defaultRedirect?: string
  }>(),
  {
    defaultRedirect: 'Profile',
  },
)
const { defaultRedirect } = toRefs(props)

const { redirectAfterLogin, currentRedirect } = useLoginRedirect({
  defaultRedirect: defaultRedirect.value,
})

const payload = ref<ILoginUserApiRequest>({
  email: '',
  password: '',
})
const loading = ref(false)

const validate = (): boolean => {
  const password = payload.value.password
  if (password.length < 8 || password.length > 50) {
    authErrorKey.value = 'errors.password_length'
    return false
  }
  return true
}

const login = async (e: Event) => {
  e.preventDefault()
  if (!validate()) {
    return
  }
  loading.value = true
  authErrorKey.value = undefined
  try {
    await loginUser(payload.value)
    loading.value = false
    if (!authErrorKey.value) {
      redirectAfterLogin()
    }
  } catch (e) {
    console.log(e)
    authErrorKey.value = errorToKey(e)
    loading.value = false
  }
  return false
}

const checkLoggedIn = () => {
  // If the user is already logged in, and there's an auth redirect, follow it
  if (store.auth.loggedIn.value && currentRedirect()) {
    redirectAfterLogin()
  }
}

const refreshStore = () => {
  store.auth.refreshData()
  checkLoggedIn()
}

onMounted(() => {
  checkLoggedIn()
  document.addEventListener('visibilitychange', refreshStore)
})

onUnmounted(() => {
  document.removeEventListener('visibilitychange', refreshStore)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.auth-subtitle {
  @mixin title-regular 16px;
  a {
    display: block;
    color: $primary;
    margin-top: 8px;
  }
}
.email-input {
  margin-top: 40px;
}
.forgot {
  text-align: right;
  padding-top: 8px;
}
.forgot-password-link {
  @mixin title 13px;
  color: $primary;
}
.password-input {
  margin-top: 12px;
}
</style>
