<template>
  <canvas ref="canvas" width="128" height="192" class="tiles" />
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { useTemplateRef } from 'vue'
const canvas = useTemplateRef('canvas')
useEmulator().useTilesCanvas(canvas)
</script>

<style lang="scss" scoped>
.tiles {
  width: 100%;
  aspect-ratio: 2 / 3;
}
</style>
