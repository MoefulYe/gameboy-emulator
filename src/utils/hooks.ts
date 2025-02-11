import { onMounted, onUnmounted, type ShallowRef, shallowRef, ref } from 'vue'

export const useElementSize = (el: Readonly<ShallowRef<HTMLElement | null>>) => {
  const size = shallowRef({
    w: 0,
    h: 0
  })
  onMounted(() => {
    const _el = el.value!
    const update = () => {
      const { offsetWidth: w, offsetHeight: h } = _el
      size.value = { w, h }
    }
    const observer = new ResizeObserver(update)
    observer.observe(_el)
    onUnmounted(() => observer.unobserve(_el))
  })
  return size
}

export const useElementClientHeight = (el: Readonly<ShallowRef<HTMLElement | null>>) => {
  const size = ref(0)
  onMounted(() => {
    const _el = el.value!
    const update = () => {
      size.value = el.value!.clientHeight
    }
    const observer = new ResizeObserver(update)
    observer.observe(_el)
    onUnmounted(() => observer.unobserve(_el))
  })
  return size
}
