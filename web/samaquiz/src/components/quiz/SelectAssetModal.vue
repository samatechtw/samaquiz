<template>
  <Modal :show="show" cls="select-asset-modal" @cancel="cancel">
    <div class="modal-title">
      {{ ts('assets.select') }}
    </div>
    <div class="modal-text">
      {{ ts('assets.asset_text') }}
    </div>
    <div class="asset-filter-row">
      <STInput
        v-model="externalUrl"
        name="external-url"
        class="external-url"
        :placeholder="ts('assets.external_url')"
      />
      <AppButton
        class="external-url-button"
        size="medium"
        :text="ts('set')"
        @click="setExternalUrl"
      />
    </div>
    <div class="divider" />
    <div class="asset-filter-row">
      <STMultiselect
        :value="quizFilter"
        valueKey="id"
        labelKey="name"
        :options="quizIdOptions"
        :clearable="false"
        class="quiz-select"
        @select="updateQuizFilter($event as string)"
      />
    </div>
    <div class="assets-wrap">
      <div v-if="assetsState.loading" class="assets-spinner-wrap f-center">
        <Spinner class="assets-spinner" :size="16" color="#2a17d6" />
      </div>
      <div v-else-if="!assetsState.assets?.length" class="no-asset">
        {{ ts('assets.none') }}
      </div>
      <div
        v-for="asset in assetsState.assets"
        v-else
        :key="asset.id"
        class="asset-item"
        :class="{ selected: initialUrl === urlFromAsset(asset) }"
      >
        <img class="asset-image" :src="urlFromAsset(asset)" />
        <div
          v-if="initialUrl !== urlFromAsset(asset)"
          class="asset-hover overlay f-center"
          @click="emitSelect(asset)"
        >
          <AppButton class="select-button" size="medium" :text="ts('select')" />
        </div>
      </div>
    </div>
  </Modal>
</template>

<script lang="ts" setup>
import { reactive, ref, watch } from 'vue'
import { STInput, STMultiselect } from '@samatech/vue-components'
import { AppButton, Modal, Spinner } from '@frontend/components/widgets'
import { IListQuizAssetParams, listAssets, urlFromAsset } from '@frontend/features'
import { AssetContentType, IQuizAssetViewModel } from '@frontend/types'
import { ts } from '../../i18n'

const assetsState = reactive<IListQuizAssetParams>({
  loading: false,
  error: undefined,
  assets: [],
  usage: 0,
})

const {
  show,
  initialUrl = undefined,
  quizId,
} = defineProps<{
  show: boolean
  quizId: string
  initialUrl?: string
  contentTypes?: AssetContentType[]
}>()

const emit = defineEmits<{
  (e: 'cancel'): void
  (e: 'select', assetUrl: string): void
}>()

const thisQuiz = ts('assets.this_quiz')
const allQuizzes = ts('assets.all_quizzes')
const quizIdOptions = [thisQuiz, allQuizzes]
const quizFilter = ref(thisQuiz)

const externalUrl = ref(initialUrl ?? '')

const setExternalUrl = () => {
  emit('select', externalUrl.value)
}

const cancel = () => {
  emit('cancel')
}

const emitSelect = (asset: IQuizAssetViewModel) => {
  if (asset) {
    emit('select', urlFromAsset(asset))
  }
}

const updateQuizFilter = (filter: string) => {
  quizFilter.value = filter
  listQuizAssets()
}

const listQuizAssets = async () => {
  const filterQuizId = quizFilter.value === thisQuiz ? quizId : undefined
  await listAssets({ quiz_id: filterQuizId }, assetsState)
}

watch(
  () => show,
  async (modalShown) => {
    externalUrl.value = initialUrl ?? ''
    if (modalShown) {
      await listQuizAssets()
    }
  },
)
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.select-asset-modal {
  .modal-inner {
    width: 480px;
    max-width: 95%;
    min-height: 320px;
    max-height: 95%;
    overflow-y: scroll;
  }
  .modal-title {
    margin: 0 0 16px;
  }
  .modal-text {
    margin-bottom: 16px;
    text-align: left;
  }
  .asset-filter-row {
    display: flex;
    align-items: center;
    margin-top: 8px;
  }
  .quiz-select {
    width: 132px;
  }
  .divider {
    width: 100%;
    height: 1px;
    background-color: $border2;
    margin: 8px 0;
  }
  .asset-content-type {
    width: 124px;
    margin-left: 10px;
  }
  .select-button {
    min-width: 56px;
    height: 32px;
    font-size: 12px;
    padding: 0 10px;
    white-space: nowrap;
  }
  .external-url {
    flex-grow: 1;
  }
  .external-url-button {
    margin-left: 8px;
    min-width: 72px;
  }
  .assets-spinner-wrap {
    width: 100%;
    height: 40px;
    margin-top: 8px;
  }
  .no-asset {
    @mixin title-thin 16px;
  }
  .assets-wrap {
    display: flex;
    flex-wrap: wrap;
    width: 100%;
    row-gap: 12px;
    column-gap: 12px;
    margin-top: 16px;
    max-height: 50vh;
    overflow: auto;
  }
  .asset-hover {
    background-color: rgba(125, 125, 125, 0.5);
    opacity: 0;
    transition: opacity 0.2s ease-in;
  }
  .asset-item {
    position: relative;
    width: calc((100% - 24px) / 3);
    height: 100px;
    display: flex;
    align-items: center;
    flex-direction: column;
    border: 2px solid $color6;
    cursor: pointer;
    &:hover .asset-hover {
      opacity: 1;
    }
    &.selected {
      border-color: $color3;
      cursor: unset;
    }
  }
  .asset-image {
    height: 100%;
    width: 100%;
    object-fit: cover;
  }
  .asset-preview {
    width: 30%;
    padding-top: 16px;
    img {
      width: 100%;
      height: 100%;
      object-fit: contain;
    }
  }
}
</style>
