<template>
  <div class="reset-password-wrap">
    <div v-if="validJwt" class="auth-wrap">
      <div class="auth-title">
        {{ ts('auth.update_password') }}
      </div>
      <div class="auth-subtitle">
        {{ ts('auth.update_password_text') }}
      </div>
      <form class="form" @submit.prevent="handleSubmit">
        <PasswordInput
          v-model="password"
          class="password-input password"
          :placeholder="ts('auth.password')"
          :isDisabled="loading"
        />
        <PasswordInput
          v-model="passwordConfirm"
          class="password-input"
          :placeholder="ts('auth.confirm_password')"
          :isDisabled="loading"
        />
        <ErrorMessage :error="ts(authErrorKey)" />
        <AppButton
          type="submit"
          class="submit-button"
          :text="ts('auth.update_password')"
          :animate="loading"
        />
      </form>
    </div>
    <div v-else class="auth-wrap">
      <div class="auth-title">
        {{ ts('auth.update_password') }}
      </div>
      <div class="auth-subtitle">
        {{ ts('auth.update_password_error') }}
        <router-link :to="{ name: 'ForgotPassword' }" class="try-again">
          {{ ts('click_here') }}
        </router-link>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref } from 'vue'
import { PasswordInput, AppButton, ErrorMessage } from '@frontend/components/widgets'
import { useRoute, useRouter } from 'vue-router'
import { ts } from '../../i18n'
import { errorToKey } from '@frontend/util/api'
import { authErrorKey, validatePassword } from '@frontend/features'
import { apiUpdatePassword } from '@frontend/api'

const route = useRoute()
const router = useRouter()

const validJwt = ref(false)

const password = ref('')
const passwordConfirm = ref('')
const loading = ref(false)

const validate = (): boolean => {
  if (!validatePassword(password.value)) {
    return false
  }
  if (password.value !== passwordConfirm.value) {
    authErrorKey.value = 'errors.password_match'
    return false
  }
  return true
}

const handleSubmit = async (e: Event) => {
  loading.value = true
  authErrorKey.value = undefined
  if (validate()) {
    const jwt = route.query?.t as string
    try {
      await apiUpdatePassword(jwt, password.value)
      router.push({ name: 'Login' })
    } catch (err) {
      console.log(err)
      authErrorKey.value = errorToKey(e)
    }
  }
  loading.value = false
}

onMounted(() => {
  const jwt = route.query?.t
  if (jwt && jwt.length > 1) {
    validJwt.value = true
  }
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.reset-password-wrap .password-input.password {
  margin-top: 40px;
}
.try-again {
  color: $primary;
}
</style>
