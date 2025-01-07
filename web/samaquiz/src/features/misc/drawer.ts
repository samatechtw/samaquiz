import { ref } from 'vue'

export const drawerActive = ref(false)

export const showDrawer = (show: boolean) => {
  drawerActive.value = show
  document.body.style.overflow = show ? 'hidden' : ''
}
