<template>
  <Modal :show="show" @cancel="emit('cancel')">
    <div class="avatar-modal-content f-col">
      <AvatarSelect v-model="avatar" />
      <AppButton
        :text="ts('select')"
        :animate="updating"
        class="select-button"
        @click="select"
      />
    </div>
  </Modal>
</template>

<script lang="ts" setup>
import { Modal, AvatarSelect, AppButton } from '@frontend/components/widgets'
import { ts } from '../../i18n'
import { ref, watch } from 'vue'
import { store } from '@frontend/store'
import { updateUser } from '@frontend/features'

const { show } = defineProps<{
  show: boolean
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
}>()

const avatar = ref()
const updating = ref(false)
const error = ref()

const select = async () => {
  if (!updating.value && avatar.value && avatar.value !== store.user.avatar.value) {
    updating.value = true
    await updateUser({ avatar: avatar.value }, error)
    updating.value = false
    emit('cancel')
  }
}

watch(
  () => show,
  (newShow) => {
    if (newShow) {
      avatar.value = store.user.avatar.value
    }
  },
)
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.avatar-modal-content {
  align-items: center;
}
.select-button {
  margin-top: 24px;
}
</style>
