<template>
  <div class="qr-code-wrap f-center-col">
    <div class="canvas-wrap">
      <canvas ref="canvasRef" />
      <slot />
    </div>
    <div class="url-wrap" @click="copy(url)">
      <div class="quiz-url">
        {{ url }}
      </div>
      <Copy class="copy" />
    </div>
  </div>
</template>

<script lang="ts" setup>
import { onMounted, ref, watch } from 'vue'
import { qrcanvas, QRCanvasOptions } from 'qrcanvas'
import { copy } from '@frontend/util/misc'
import Copy from './Copy.vue'

const { url, size } = defineProps<{
  url: string
  size: number
}>()

const canvasRef = ref()

const update = () => {
  const options: QRCanvasOptions = {
    data: url,
    size,
    canvas: canvasRef.value,
  }
  qrcanvas(options)
}

watch(() => size, update)

onMounted(() => {
  update()
})
</script>

<style lang="postcss" scoped>
@import '@theme/css/defines.postcss';

.canvas-wrap {
  position: relative;
}

$url-color: $color2;
$url-hover: $color3;
.url-wrap {
  @mixin title-regular 15px;
  display: flex;
  align-items: center;
  cursor: pointer;
  color: $url-color;
  :deep(path) {
    fill: $url-color;
  }
  &:hover {
    color: $url-hover;
    :deep(path) {
      fill: $url-hover;
    }
  }
  margin-top: 10px;
}
.copy {
  @mixin size 18px;
  margin-left: 6px;
}
</style>
