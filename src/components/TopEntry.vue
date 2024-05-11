<template>
  <div class="w-full h-full flex flex-col min-h-screen">
    <HeaderBar />
    <!-- <main class="flex-grow p-4">
      <EmulatorMain />
      <div ref="devTools">sss</div>
    </main> -->
    <div style="height: 500px; width: 500px; border: 1px solid red; position: relative">
      <VueDraggableResizable :w="100" :h="100" :parent="true" :resizeable="true">
        <p>Hello! I'm a flexible component. You can drag me around and you can resize me.</p>
      </VueDraggableResizable>
    </div>
  </div>
</template>

<script setup lang="ts">
import { getEmulator, emuKey } from '@/emulator'
import sleep from '@/utils/sleep'
import { provide, shallowRef } from 'vue'
import HeaderBar from './HeaderBar.vue'
import EmulatorMain from './EmulatorMain.vue'
import EmulatorDevTools from './EmulatorDevTools.vue'
// @ts-ignore
import VueDraggableResizable from 'vue-draggable-resizable'

const props = defineProps<{
  delay: number
}>()

const delay = sleep(props.delay)
const emulator = await getEmulator()
await delay
provide(emuKey, emulator)

const devTools = shallowRef<HTMLElement>()
</script>

<style lang="scss" scoped>
@import 'vue-draggable-resizable/style.css';
</style>
