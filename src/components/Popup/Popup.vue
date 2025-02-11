<template>
  <div class="--popup font-sans">
    <div
      ref="content"
      class="bg-coolgray-50 w-5/6 h-11/12 max-w-4xl max-h-6xl rounded-lg shadow-lg p-4"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted, useTemplateRef } from 'vue'

type Emits = {
  close: []
}
const emit = defineEmits<Emits>()
const content = useTemplateRef('content')
const onclick = (e: MouseEvent) => {
  const ok = content.value!.contains(e.target as any)
  if (!ok) {
    emit('close')
  }
}
onMounted(() => window.addEventListener('click', onclick))
onUnmounted(() => window.removeEventListener('click', onclick))
</script>

<style lang="scss" scoped>
.--popup {
  z-index: 10;
  position: absolute;
  inset: 0;
  backdrop-filter: brightness(80%) blur(2px);
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
