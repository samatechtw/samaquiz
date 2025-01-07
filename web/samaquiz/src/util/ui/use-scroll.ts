import { onMounted, onUnmounted, ref } from 'vue'

export const useScroll = () => {
  const yPos = ref(0)

  const scrollHandler = () => {
    yPos.value = window.pageYOffset ?? window.scrollY
  }

  onMounted(() => {
    document.addEventListener('scroll', scrollHandler, { passive: true })
  })

  onUnmounted(() => {
    document.removeEventListener('scroll', scrollHandler)
  })

  return { yPos }
}
