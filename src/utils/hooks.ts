import { onMounted, onUnmounted, ref, type Ref } from 'vue'

export const useWindowWidth = (): Ref<number> => {
  const width = ref(window.innerWidth)

  const update = () => (width.value = window.innerWidth)

  onMounted(() => window.addEventListener('resize', update))
  onUnmounted(() => window.removeEventListener('resize', update))

  return width
}
