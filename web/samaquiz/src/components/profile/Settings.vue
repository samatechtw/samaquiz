<template>
  <div class="settings-wrap">
    <SettingsRow :title="ts('email')" :text="ts('profile.email_text')">
      <template #right>
        <div v-if="emailUpdate" class="email-right f-col">
          <AppInput v-model="newEmail" :placeholder="ts('profile.new_email')" />
          <ErrorMessage :error="errorKey && ts(errorKey)" />
          <div class="update-buttons">
            <div class="email-cancel cancel" @click="emailUpdate = false">
              {{ ts('cancel') }}
            </div>
            <AppButton
              :text="ts('save')"
              :animate="saving"
              class="email-button settings-button"
              @click="saveEmail"
            />
          </div>
        </div>
        <div v-else class="email-right f-col">
          <span class="email">{{ email }}</span>
          <AppButton
            :text="ts('profile.update_email')"
            class="email-button settings-button"
            :disabled="passwordUpdate"
            @click="showEmailUpdate"
          />
        </div>
      </template>
    </SettingsRow>
    <SettingsRow :title="ts('password')" :text="ts('profile.password_text')">
      <template #right>
        <div v-if="passwordUpdate" class="password-right f-col">
          <PasswordInput
            v-model="oldPassword"
            :placeholder="ts('profile.old_password')"
            class="password-input"
          />
          <PasswordInput
            v-model="newPassword"
            :placeholder="ts('profile.new_password')"
            class="password-input"
          />
          <PasswordInput
            v-model="confirmPassword"
            :placeholder="ts('profile.confirm_password')"
            class="password-input"
          />
          <ErrorMessage :error="errorKey && ts(errorKey)" />
          <div class="update-buttons">
            <div class="password-cancel cancel" @click="passwordUpdate = false">
              {{ ts('cancel') }}
            </div>
            <AppButton
              :text="ts('save')"
              :animate="saving"
              class="password-button settings-button"
              @click="savePassword"
            />
          </div>
        </div>
        <div v-else class="password-right f-col">
          <AppButton
            :text="ts('profile.update_password')"
            :disabled="emailUpdate"
            class="settings-button"
            @click="showPasswordUpdate"
          />
        </div>
      </template>
    </SettingsRow>
  </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { updateUser } from '@frontend/features'
import { store } from '@frontend/store'
import { IUpdateUserApiRequest } from '@frontend/types'
import {
  AppButton,
  AppInput,
  ErrorMessage,
  PasswordInput,
} from '@frontend/components/widgets'
import { ts } from '../../i18n'
import SettingsRow from './SettingsRow.vue'

const emailUpdate = ref(false)
const passwordUpdate = ref(false)

const newEmail = ref('')
const oldPassword = ref('')
const newPassword = ref('')
const confirmPassword = ref('')
const saving = ref(false)
const errorKey = ref()

const email = computed(() => {
  return store.user.email.value || 'No email'
})

const showEmailUpdate = () => {
  errorKey.value = undefined
  emailUpdate.value = true
  newEmail.value = store.user.email.value || ''
}

const saveEmail = async () => {
  const payload: IUpdateUserApiRequest = {
    email: newEmail.value,
  }
  saving.value = true
  await updateUser(payload, errorKey)
  saving.value = false
  if (!errorKey.value) {
    emailUpdate.value = false
  }
}

const showPasswordUpdate = () => {
  errorKey.value = undefined
  passwordUpdate.value = true
}

const savePassword = async () => {
  if (newPassword.value !== confirmPassword.value) {
    errorKey.value = 'errors.confirm_password'
    return
  }
  const payload: IUpdateUserApiRequest = {
    old_password: oldPassword.value,
    new_password: newPassword.value,
  }
  saving.value = true
  await updateUser(payload, errorKey)
  saving.value = false
  if (!errorKey.value) {
    passwordUpdate.value = false
  }
}
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.settings-wrap {
  min-height: 240px;
}
.email {
  @mixin text 15px;
  text-align: right;
  margin-bottom: 12px;
  color: $text2;
}
.settings-button {
  width: 172px;
  color: $text2;
}
.email-cancel {
  @mixin title-regular 14px;
  color: $red;
  margin-right: 8px;
  cursor: pointer;
}
.update-buttons {
  display: flex;
  align-items: center;
  margin-top: 16px;
}
.password-input {
  margin-bottom: 8px;
}
</style>
