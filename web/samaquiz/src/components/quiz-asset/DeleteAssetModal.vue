<template>
  <Modal :show="show" cls="delete-asset-modal" @cancel="closeModal">
    <div class="modal-title">
      {{ t('delete') }}
    </div>
    <div class="modal-text">
      {{ t('assets.delete_text') }}
    </div>
    <div class="modal-buttons">
      <PSButton
        class="confirm-button"
        :text="t('confirm')"
        :animate="loading"
        @click="execDeleteAsset"
      />
      <PSButton
        class="cancel-button"
        :text="t('cancel')"
        :secondary="true"
        @click="closeModal"
      />
    </div>
    <ErrorMessage :error="error" />
  </Modal>
</template>

<script lang="ts" setup>
import { ref, toRefs } from 'vue'
import { ErrorMessage, Modal, PSButton } from '@pubstudio/frontend/ui-widgets'
import { parseApiError, toApiError } from '@pubstudio/frontend/util-api'
import { useQuizAssets } from '../lib/use-quiz-assets'
import { ts } from '../../i18n'

const { deleteAsset } = useQuizAssets()

const props = defineProps<{
  show: boolean
  assetId: string
}>()

const { assetId } = toRefs(props)

const emit = defineEmits<{
  (e: 'deleted'): void
  (e: 'cancel'): void
}>()

const error = ref<string>()
const loading = ref(false)

const closeModal = () => {
  error.value = undefined
  emit('cancel')
}

const execDeleteAsset = async () => {
  error.value = undefined
  loading.value = true
  try {
    await deleteAsset(assetId.value)
    emit('deleted')
  } catch (e) {
    console.log(e)
    error.value = ts('errors.None')
  }
  loading.value = false
}
</script>

<style lang="postcss">
@import '@theme/css/mixins.postcss';

.delete-asset-modal {
  .modal-buttons {
    @mixin flex-row;
    .confirm-button {
      margin-right: 8px;
      min-width: 94px;
    }
    .cancel-button {
      margin-left: 8px;
    }
  }
}
</style>
