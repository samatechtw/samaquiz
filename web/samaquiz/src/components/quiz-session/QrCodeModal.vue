<template>
  <Modal :show="show" cls="qr-code-modal" @cancel="emit('cancel')">
    <QrCode :url="url" :size="size" class="qr-modal-content" />
  </Modal>
</template>

<script lang="ts" setup>
import { Modal, QrCode } from '@frontend/components/widgets'
import { onMounted, ref, watch } from 'vue'

const { show } = defineProps<{
  show: boolean
  url: string
}>()
const emit = defineEmits<{
  (e: 'cancel'): void
}>()

const size = ref(400)

const updateSize = () => {
  const max = Math.min(window.innerHeight, window.innerWidth)
  size.value = Math.round(0.8 * max)
}

watch(() => show, updateSize)

onMounted(() => {
  updateSize()
})
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.qr-code-modal {
  .modal-inner {
    width: 100%;
    height: 100%;
    border-radius: 0;
    padding: 0;
  }
  .qr-modal-content {
    height: 100%;
  }
}
</style>
