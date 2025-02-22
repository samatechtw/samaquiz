import { onMounted, ref } from 'vue'

let retries = 0
let timer: ReturnType<typeof setInterval> | undefined = undefined

export const kofiHtml = ref()

export const useKofiButton = () => {
  const checkButton = () => {
    const kofi = window.kofiwidget2
    if (kofi) {
      clearInterval(timer)
      timer = undefined
      kofi.init('Support me on Ko-fi', 'rgb(136, 167, 167)', 'I2I01AYU23')
      kofiHtml.value = kofi.getHTML()
    } else if (retries >= 3) {
      clearInterval(timer)
      timer = undefined
      console.log('Failed to load Ko-Fi')
    } else {
      retries += 1
    }
  }

  onMounted(() => {
    if (!kofiHtml.value && timer === undefined) {
      timer = setInterval(checkButton, 1000)
      checkButton()
    }
  })
}
