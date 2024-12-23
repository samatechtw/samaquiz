<template>
  <Modal :show="show" cls="edit-profile-modal">
    <div class="edit-profile f-center-col">
      <ModalClose class="edit-close" @click="emit('cancel')" />
      <div class="modal-title">
        {{ ts('profile.edit') }}
      </div>
      <AppInput v-model="name" :label="ts('name')" class="edit-input" />
      <AppTextArea v-model="bio" :label="ts('bio')" class="edit-input" />
      <AppInput v-model="location" :label="ts('location')" class="edit-input" />
      <AppInput v-model="link" :label="ts('link')" class="edit-input" />
      <div class="edit-buttons">
        <AppButton
          :text="ts('save')"
          :animate="saving"
          class="edit-submit button2"
          @click="save"
        />
        <AppButton
          :text="ts('cancel')"
          class="button2 button-cancel"
          @click="emit('cancel')"
        />
      </div>
    </div>
  </Modal>
</template>

<script setup lang="ts">
import { onMounted, ref, watch } from 'vue'
import { updateUser } from '@frontend/features'
import { store } from '@frontend/store'
import { IUpdateUserApiRequest } from '@frontend/types'
import {
  AppInput,
  AppTextArea,
  AppButton,
  Modal,
  ModalClose,
} from '@frontend/components/widgets'
import { ts } from '../../i18n'

const { show } = defineProps<{
  show: boolean
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
}>()

const name = ref('')
const bio = ref('')
const location = ref('')
const link = ref('')
const errorKey = ref('')
const saving = ref(false)

const initialize = () => {
  name.value = store.user.name.value || 'No Name'
  bio.value = store.user.description.value || ''
  location.value = store.user.location.value || ''
  link.value = store.user.link.value || ''
}

watch(
  () => show,
  (newShow) => {
    if (newShow) {
      initialize()
    }
  },
)

onMounted(() => {
  initialize()
})

const validate = (): boolean => {
  if (name.value.length < 2 || name.value.length > 20) {
    errorKey.value = 'errors.name_len'
    return false
  } else if (bio.value.length > 400) {
    errorKey.value = 'errors.bio_len'
    return false
  }
  return true
}

const save = async () => {
  if (!validate()) {
    return
  }
  const payload: IUpdateUserApiRequest = {
    name: name.value,
    description: bio.value,
    location: location.value,
    link: link.value,
  }
  saving.value = true
  await updateUser(payload, errorKey)
  saving.value = false
  if (!errorKey.value) {
    emit('cancel')
  }
}
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.edit-profile-modal {
  .modal-inner {
    width: 100%;
    height: 100%;
    border-radius: 0;
  }
  .edit-profile {
    max-width: 720px;
    padding: 96px 32px 80px;
    margin: 0 auto;
    position: relative;
  }
  .modal-close {
    display: none;
  }
  .modal-title {
    margin-bottom: 24px;
  }
  .edit-input {
    margin: 16px auto 0;
    width: 100%;
    max-width: 420px;
  }
  .edit-close {
    display: block;
    top: 80px;
  }
  .edit-submit {
    width: 110px;
    margin: 0 24px 0 0;
    border: 1px solid $primary;
  }
  .button-cancel {
    width: 110px;
  }
  .edit-buttons {
    margin-top: 24px;
    display: flex;
  }
}
</style>
