<template>
  <Modal :show="show" cls="delete-account-modal" @cancel="emit('cancel')">
    <div class="delete-account f-center-col">
      <div class="modal-title">
        {{ ts('profile.delete') }}
      </div>
      <div class="modal-text">
        {{ ts('profile.delete_confirm1') }}
      </div>
      <div class="modal-text confirm2">
        {{ ts('profile.delete_confirm2') }}
      </div>
      <AppInput v-model="confirmText" :placeholder="ts('delete')" class="confirm-input" />
      <ErrorMessage :error="error" />
      <div class="delete-buttons">
        <AppButton
          :text="ts('confirm')"
          :animate="deleting"
          class="confirm-button button-red"
          @click="deleteAccount"
        />
        <AppButton
          :text="ts('cancel')"
          class="button2 cancel-button"
          @click="emit('cancel')"
        />
      </div>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useLoginRedirect } from '@frontend/features'
import { AppButton, AppInput, ErrorMessage, Modal } from '@frontend/components/widgets'
import { store } from '@frontend/store'
import { apiDeleteUser } from '@frontend/api'
import { ts } from '../../i18n'

const { show } = defineProps<{
  show: boolean
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
}>()

const { checkAuthRedirect } = useLoginRedirect()

const deleting = ref(false)
const error = ref()
const confirmText = ref('')

const deleteAccount = async () => {
  if (confirmText.value.toLowerCase() !== 'delete') {
    error.value = ts('errors.confirm_text')
    return
  }
  error.value = undefined
  deleting.value = true
  try {
    await apiDeleteUser(store.user.id.value)
    store.auth.logOut()
    checkAuthRedirect()
  } catch (e) {
    error.value = ts('errors.None')
  }
  deleting.value = false
}
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.delete-account-modal {
  .confirm-button {
    width: 110px;
    margin: 0 24px 0 0;
  }
  .confirm2 {
    margin-top: 4px;
  }
  .confirm-input {
    margin-top: 20px;
  }
  .cancel-button {
    width: 110px;
  }
  .delete-buttons {
    margin-top: 24px;
    display: flex;
  }
}
</style>
