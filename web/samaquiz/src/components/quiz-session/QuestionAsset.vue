<template>
  <div v-if="questionAsset" class="question-asset-wrap">
    <div class="question-asset" :style="{ 'background-image': questionAsset }"></div>
  </div>
</template>

<script lang="ts" setup>
import { IQuestionViewModel } from '@frontend/types'
import { questionAssetUrl } from '@frontend/util/ui'
import { computed } from 'vue'

const { question } = defineProps<{
  question: IQuestionViewModel
}>()

const questionAsset = computed(() => {
  return questionAssetUrl(question)
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.question-asset-wrap {
  display: flex;
  height: 320px;
  width: 100%;
  justify-content: center;
  margin-top: 16px;
}
.question-asset {
  height: 100%;
  background-position: center;
  background-size: cover;
  aspect-ratio: 2;
}
</style>
