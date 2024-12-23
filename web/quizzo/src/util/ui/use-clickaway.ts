import { onMounted, onUnmounted } from 'vue'

const clickEventType = document.ontouchstart !== null ? 'click' : 'touchend'

export interface IClickaway {
  activate: () => void
  deactivate: () => void
}

export type ClickawayCallback = (event: Event) => void

export const createClickawayListener = (
  ignoreElementsSelector: string,
  callback: ClickawayCallback,
) => {
  return (event: Event) => {
    const ignore = Array.from(document.querySelectorAll(ignoreElementsSelector)).some(
      (el: Element) => {
        const eventEl = event.target as Element
        const isTarget = el.contains(eventEl)
        if (isTarget) {
          return true
        } else {
          return !!eventEl.closest(ignoreElementsSelector)
        }
      },
    )
    if (!ignore) {
      return callback(event)
    }
    return null
  }
}

export const useClickaway = (
  ignoreElementsSelector: string,
  callback: ClickawayCallback,
  manual = false,
): IClickaway => {
  let listener: EventListener

  const activate = () => {
    document.addEventListener(clickEventType, listener, false)
  }

  const deactivate = () => {
    if (listener) {
      document.removeEventListener(clickEventType, listener, false)
    }
  }

  const mounted = () => {
    listener = createClickawayListener(ignoreElementsSelector, callback)
    if (!manual) {
      activate()
    }
  }

  onMounted(() => {
    mounted()
  })
  onUnmounted(() => {
    deactivate()
  })

  return { activate, deactivate }
}
