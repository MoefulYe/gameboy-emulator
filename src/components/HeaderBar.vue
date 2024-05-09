<template>
  <header class="bg-blue-3 text-white text-5xl shadow-sm flex gap-2 border-b-1 p-2 justify-center">
    <span class="i-pixelarticons:folder" />
    <span class="i-pixelarticons:save" />
    <span class="border-l-2 mx-1 my-2 md:mx-4" />
    <span class="i-pixelarticons:close"></span>
    <span class="i-pixelarticons:reload"></span>
    <span class="i-pixelarticons:play" />
    <span class="i-pixelarticons:pause text-gray-2" />
    <span class="i-pixelarticons:next" />
    <span class="border-l-2 mx-1 my-2 md:mx-4" />
    <span class="i-pixelarticons:speed-medium" />
    <span class="i-pixelarticons:volume-2" />
    <span class="i-pixelarticons:camera" />
    <span class="i-pixelarticons:debug" />
    <span class="i-pixelarticons:more-horizontal" />
  </header>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'

const emu = useEmulator()
</script>
