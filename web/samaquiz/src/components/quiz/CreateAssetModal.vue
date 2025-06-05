<template>
  <Modal :show="show" cls="create-asset-modal" @cancel="cancel">
    <div class="modal-title">
      {{ ts('assets.new') }}
    </div>
    <div class="modal-text">
      {{ ts('assets.new_text') }}
    </div>
    <div class="upload-wrap">
      <UploadFile
        class="create-asset-file-picker"
        :preview="assetBase64"
        accept="image/*"
        @fileSelect="updateAsset"
      >
      </UploadFile>
    </div>
    <ErrorMessage :error="error" :interpolationKey="errorInterpolationKey">
      <router-link :to="{ name: 'Assets' }" target="_blank">
        {{ ts('assets.title') }}
      </router-link>
    </ErrorMessage>
    <div class="button-wrap">
      <AppButton
        class="create-button"
        size="medium"
        :text="ts('upload')"
        :animate="loading"
        @click="uploadAsset"
      />
    </div>
  </Modal>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue'
import { ts } from '../../i18n'
import { ALL_CONTENT_TYPES, ICreateQuizAssetRequest } from '@frontend/types'
import { IUploadFileResult } from '@frontend/features'
import {
  createAsset,
  IValidateMediaError,
  ValidatedFile,
  validateMedia,
  verifyAsset,
} from '@frontend/features'
import { AppButton, ErrorMessage, Modal, UploadFile } from '@frontend/components/widgets'

interface ISelectableSite {
  id: string
  name: string
  [key: string]: unknown
}

const { quizId } = defineProps<{
  show: boolean
  quizId: string
}>()

const emit = defineEmits<{
  (e: 'cancel'): void
  (e: 'complete', asset: IUploadFileResult): void
}>()

const name = ref('')
const error = ref()

const errorInterpolationKey = computed(() => {
  const key = 'errors.AssetUsageExceeded'
  if (error.value === ts(key)) {
    return key
  } else {
    return undefined
  }
})
const assetBase64 = ref('')
const loading = ref(false)
const validatedFile = ref<ValidatedFile | undefined>()

const handleAssetSelect = (validFile: ValidatedFile) => {
  const file = validFile.file

  const reader = new FileReader()
  reader.readAsDataURL(file)
  reader.onload = () => {
    assetBase64.value = reader.result?.toString() ?? ''
  }

  validatedFile.value = validFile
}

const updateAsset = async (file: File | null | undefined) => {
  if (file) {
    error.value = undefined
    try {
      const validFile = await validateMedia(
        { size: 10000000, types: ALL_CONTENT_TYPES },
        file,
      )
      handleAssetSelect(validFile)
    } catch (e) {
      const key = (e as IValidateMediaError).fileErrors[0]
      error.value = ts(`errors.${key}`)
    }
  }
}

const uploadCreateAsset = async (validatedFile: ValidatedFile) => {
  const payload: ICreateQuizAssetRequest = {
    content_size: validatedFile.file.size,
    content_type: validatedFile.type,
    quiz_id: quizId,
  }
  const result = await createAsset(payload, validatedFile.file)
  const success = await verifyAsset(result.id)
  if (success) {
    complete(result)
  } else {
    error.value = ts('errors.asset_upload_failed')
  }
}

const uploadAsset = async () => {
  if (!validatedFile.value) {
    if (!error.value) {
      error.value = ts('errors.invalid_file')
    }
    return
  }
  loading.value = true
  try {
    await uploadCreateAsset(validatedFile.value)
  } catch (e) {
    error.value = ts('errors.None')
  } finally {
    loading.value = false
  }
}

const cleanup = () => {
  error.value = undefined
  validatedFile.value = undefined
  assetBase64.value = ''
}

const cancel = () => {
  cleanup()
  emit('cancel')
}

const complete = (result: IUploadFileResult) => {
  cleanup()
  emit('complete', result)
}
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.create-asset-modal {
  .modal-inner {
    width: 480px;
    max-width: 95%;
    max-height: 95%;
    overflow-y: scroll;
  }

  .modal-text {
    margin: 16px 0 16px;
    font-weight: 400;
  }

  .button-wrap {
    display: flex;
    margin-top: 16px;
    justify-content: center;
  }
  .create-button {
    min-width: 80px;
    width: 80px;
    white-space: nowrap;
  }
  .upload-wrap {
    margin-top: 16px;
  }
  .asset-preview {
    width: 40%;
    background-color: white;
  }
}
</style>
