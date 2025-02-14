import { onMounted, onUnmounted, type ShallowRef, shallowRef, ref } from 'vue'

export const useElementSize = (el: Readonly<ShallowRef<HTMLElement | null>>) => {
  const size = shallowRef({
    w: 0,
    h: 0
  })
  const update = () => {
    const { offsetWidth: w, offsetHeight: h } = el.value!
    size.value = { w, h }
  }
  const observer = new ResizeObserver(update)
  onMounted(() => observer.observe(el.value!))
  onUnmounted(() => observer.unobserve(el.value!))
  return size
}

export const useElementClientHeight = (el: Readonly<ShallowRef<HTMLElement | null>>) => {
  const size = ref(0)
  const update = () => {
    size.value = el.value!.clientHeight
  }
  const observer = new ResizeObserver(update)
  onMounted(() => observer.observe(el.value!))
  onUnmounted(() => observer.unobserve(el.value!))
  return size
}

export const useWindowListener = <Event extends keyof WindowEventMap>(
  event: Event,
  callback: (this: Window, ev: WindowEventMap[Event]) => unknown,
  options?: boolean | AddEventListenerOptions
) => {
  onMounted(() => window.addEventListener(event, callback, options))
  onUnmounted(() => window.removeEventListener(event, callback))
}

export const useDocumentListener = <Event extends keyof DocumentEventMap>(
  event: Event,
  callback: (this: Document, ev: DocumentEventMap[Event]) => unknown,
  options?: boolean | AddEventListenerOptions
) => {
  onMounted(() => document.addEventListener(event, callback, options))
  onUnmounted(() => document.removeEventListener(event, callback))
}
