<template>
  <div class="confirm-email auth-wrap">
    <div class="auth-title">
      {{ ts('auth.confirm') }}
    </div>
    <Spinner v-if="confirmState.loading" :size="28" class="spinner f-center" />
    <div v-else class="auth-subtitle">
      <div v-if="confirmState.expired">
        <div v-if="resendState.complete">
          <div>
            {{ ts('auth.confirm_resent', { email: store.user.email.value }) }}
          </div>
        </div>
        <div v-else-if="store.auth.loggedIn.value">
          <div>
            {{ ts('auth.confirm_expired') }}
          </div>
          <AppButton
            :text="ts('auth.resend')"
            :animate="resendState.loading"
            @click="resendConfirm"
          />
          <ErrorMessage :error="ts(resendState.errorKey)" />
        </div>
        <div v-else>
          <div>
            {{ ts('auth.confirm_expired_login') }}
          </div>
          <router-link :to="{ name: 'Settings' }">
            <AppButton :text="ts('settings.title')" />
          </router-link>
        </div>
      </div>
      <div v-else-if="confirmState.errorKey">
        <div>{{ `${ts(confirmState.errorKey)} ${ts('auth.confirm_again')}` }}</div>
        <router-link :to="{ name: 'Settings' }">
          <AppButton :text="ts('settings.title')" />
        </router-link>
      </div>
      <div v-else>
        <div>
          {{ ts('auth.confirmed') }}
        </div>
        <router-link :to="{ name: 'Profile' }">
          <AppButton :text="ts('profile.title')" />
        </router-link>
      </div>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue'
import { useRoute } from 'vue-router'
import { ErrorMessage, AppButton, Spinner } from '@frontend/components/widgets'
import {
  confirmEmail,
  confirmState,
  resendState,
  resendConfirm,
} from '@frontend/features'
import { ts } from '../../i18n'
import { store } from '@frontend/store'

const route = useRoute()

onMounted(async () => {
  const code = (route.query?.code || '').toString()
  await confirmEmail(code)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.confirm-email {
  align-items: center;
}
.spinner {
  margin-top: 48px;
}
.auth-subtitle {
  max-width: 440px;
}
:deep(.p-button) {
  margin-top: 24px;
}
:deep(.error-message .error) {
  justify-content: center;
}
</style>
