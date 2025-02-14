<template>
  <div class="--popup font-sans">
    <div
      ref="content"
      class="bg-coolgray-50 w-5/6 h-11/12 max-w-4xl max-h-6xl rounded-lg shadow-lg p-4"
    ></div>
  </div>
</template>

<script setup lang="ts">
import { useWindowListener } from '@/utils/hooks'
import { useTemplateRef } from 'vue'
const emit = defineEmits<Emits>()
const content = useTemplateRef('content')
useWindowListener('click', (e) => {
  const ok = content.value!.contains(e.target as any)
  if (!ok) {
    emit('close')
  }
})
</script>

<script lang="ts">
type Emits = {
  close: []
}
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
