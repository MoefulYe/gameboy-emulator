import { onMounted, onUnmounted, ref, type Ref, type ComputedRef, computed } from 'vue'

export const useWindowWidth = (): Ref<number> => {
  const width = ref(window.innerWidth)

  const update = () => (width.value = window.innerWidth)

  onMounted(() => window.addEventListener('resize', update))
  onUnmounted(() => window.removeEventListener('resize', update))

  return width
}

export const useNotMobile = (): ComputedRef<boolean> => {
  const MD = 768
  const width = useWindowWidth()
  return computed(() => width.value >= MD)
}
