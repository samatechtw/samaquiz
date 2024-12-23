<template>
  <Teleport to="body">
    <transition name="modal">
      <div
        v-if="show"
        class="modal-outer overlay f-center"
        :class="cls"
        @mousedown="clickOutside"
      >
        <div class="modal-inner">
          <ModalClose
            v-if="!disableClose"
            :color="closeColor"
            @click="emit('cancel')"
            class="modal-close"
          />
          <slot />
        </div>
      </div>
    </transition>
  </Teleport>
</template>

<script lang="ts" setup>
import { onMounted, onUnmounted, ref, toRefs, watch } from 'vue'
import ModalClose from './ModalClose.vue'

const props = withDefaults(
  defineProps<{
    show?: boolean
    cls?: string
    closeColor?: string
    disableClose?: boolean
    cancelByClickingOutside?: boolean
    cancelWithEscape?: boolean
    escapeEvent?: 'keyup' | 'keydown'
  }>(),
  {
    closeColor: 'black',
    cls: undefined,
    cancelByClickingOutside: true,
    cancelWithEscape: true,
    escapeEvent: 'keydown',
  },
)
const { show, cancelByClickingOutside, escapeEvent, cancelWithEscape } = toRefs(props)
const emit = defineEmits<{
  (e: 'cancel'): void
}>()

const listenersActive = ref(false)

function clickOutside(e: Event) {
  if (
    e.target &&
    e.target instanceof Element &&
    typeof e.target.className === 'string' &&
    e.target.className.split(' ').indexOf('modal-outer') >= 0 &&
    cancelByClickingOutside.value
  ) {
    emit('cancel')
  }
}
function escape(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    if (cancelWithEscape.value && e.key === 'Escape') {
      emit('cancel')
    }
  }
}

const modalSetup = (show: boolean) => {
  if (show === listenersActive.value) {
    return
  }
  if (show) {
    document.addEventListener(escapeEvent.value, escape)
    document.body.classList.add('noscroll')
  } else {
    document.removeEventListener(escapeEvent.value, escape)
    document.body.classList.remove('noscroll')
  }
  listenersActive.value = show
}

watch(show, modalSetup)

onMounted(() => {
  modalSetup(show.value)
})

onUnmounted(() => {
  modalSetup(false)
})
</script>

<style lang="postcss">
@import '@theme/css/defines.postcss';

.modal-outer {
  font-family: $font-text;
  background-color: rgba(0, 0, 0, 0.6);
  position: fixed;
  z-index: 5000;
}
.modal-inner {
  position: relative;
  padding: 24px 24px 24px;
  background-color: white;
  width: 440px;
  border-radius: 4px;
}

.modal-enter-active,
.modal-leave-active {
  transition: opacity 0.3s linear;
}
.modal-enter-from,
.modal-leave-to {
  opacity: 0;
}

.modal-enter-active .modal-inner,
.modal-leave-active .modal-inner {
  transition:
    transform 0.3s cubic-bezier(0.5, 0, 0.5, 1),
    opacity 0.3s linear;
}
.modal-enter-from .modal-inner,
.modal-leave-to .modal-inner {
  opacity: 0;
  transform: scale(0.7) translateY(-10%);
}

@media (max-width: 580px) {
  .modal-outer {
    align-items: flex-start;
  }
  .modal-inner {
    top: 10px;
  }
}
</style>
