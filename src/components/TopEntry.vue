<template>
  <div class="w-full h-full flex flex-col min-h-screen">
    <HeaderBar />
    <div class="grow flex p-2 items-stretch">
      <main class="grow">
        <EmulatorMain />
      </main>
      <aside
        v-if="notMobile"
        v-resizable="ASIDE_RESIZABLE_CONFIG"
        class="bg-gray-50 w-1/2 border-2 border-gray-1 rounded-lg shadow-sm p-2 flex-(~ col) gap-2 font-mono"
      >
        <div class="grow bg-white rounded-md border p-2">
          <EmulatorDevTools />
        </div>
        <div v-resizable="LOGOUTPUT_RESIZABLE_CONFIG" class="h-2/5 bg-white rounded-md border">
          <EmulatorLogOutput />
        </div>
      </aside>
    </div>
  </div>
</template>

<script setup lang="ts">
import { provide } from 'vue'
import { getEmulator, emuKey } from '@/emulator'
import { vResizable, type ResizableConfig } from 'vue-resizables'
import sleep from '@/utils/sleep'
import HeaderBar from './HeaderBar.vue'
import EmulatorMain from './EmulatorMain.vue'
import EmulatorDevTools from './EmulatorDevTools.vue'
import { useNotMobile } from '@/utils/hooks'
import 'vue-resizables/style'
import EmulatorLogOutput from './EmulatorLogOutput.vue'
const props = defineProps<{
  delay: number
}>()
const delay = sleep(props.delay)
const emulator = await getEmulator()
await delay
provide(emuKey, emulator)

const notMobile = useNotMobile()
</script>

<script lang="ts">
const ASIDE_RESIZABLE_CONFIG: ResizableConfig = {
  edge: {
    left: true
  },
  border: true,
  size: {
    min: {
      width: 200
    }
  }
}

const LOGOUTPUT_RESIZABLE_CONFIG: ResizableConfig = {
  edge: {
    top: true
  },
  border: true,
  size: {
    min: {
      height: 200
    }
  }
}
</script>
