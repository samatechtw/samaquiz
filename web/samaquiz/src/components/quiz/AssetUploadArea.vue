<template>
  <div class="asset-upload-area f-col">
    <div class="image-label">
      {{ ts('quiz.image_label') }}
    </div>
    <div class="image-text">
      {{ ts('quiz.image_text') }}
    </div>
    <div class="image-buttons" :style="{ 'background-image': imageUrl }">
      <AppButton
        :text="ts('upload')"
        class="upload-button"
        :animate="updating"
        @click="emit('upload')"
      />
      <AppButton
        :text="ts('select')"
        class="select-button"
        :animate="updating"
        @click="emit('select')"
      />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { AppButton } from '@frontend/components/widgets'
import { ts } from '../../i18n'

defineProps<{
  updating: boolean
  imageUrl: string | undefined
}>()

const emit = defineEmits<{
  (e: 'upload'): void
  (e: 'select'): void
}>()
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.asset-upload-area {
  padding: 4px 24px 0;
  text-align: center;
  align-items: center;
}
.image-label {
  @mixin title 15px;
}
.image-text {
  @mixin title-thin 14px;
  margin-top: 4px;
  max-width: 240px;
}
.image-buttons {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  flex-grow: 1;
  background-position: center;
  background-size: cover;
  margin-top: 8px;
  min-height: 132px;
  background-color: rgba($color3, 0.08);
}
.select-button {
  margin-left: 24px;
}
</style>
