<template>
  <div class="text-gray-8 flex flex-col">
    <nav class="relative px-2 overflow-x-auto flex gap-4 text-lg lg:text-xl border-b no-scroll-bar">
      <button
        v-for="{ title, icon, idx } of infos"
        :key="idx"
        :class="[
          'p-2 flex items-center gap-2',
          idx !== tabIdx
            ? 'hover:(border-blue-3 text-blue-3)'
            : 'border-b-2 border-blue-5 text-blue-5'
        ]"
        @click="tabIdx = idx"
      >
        <span :class="['text-xl lg:text-2xl', icon]" />
        <span>{{ title }}</span>
      </button>
    </nav>
    <main class="grow overflow-y-auto p-4 text-base lg:text-xl">
      <KeepAlive>
        <component :is="tab" />
      </KeepAlive>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { computed } from 'vue'
import General from './dev-tools/General.vue'
import SerialOutput from './dev-tools/SerialOutput.vue'
import CartridgeInfo from './dev-tools/CartridgeInfo.vue'
const components = [General, CartridgeInfo, General, SerialOutput]
const tabIdx = ref(0)
const tab = computed(() => components[tabIdx.value])
</script>

<script lang="ts">
interface TabInfo {
  title: string
  icon: string
  idx: number
}
const infos: TabInfo[] = [
  {
    title: 'General',
    icon: 'i-solar:settings-minimalistic-outline',
    idx: 0
  },
  {
    title: 'Cart',
    icon: 'i-solar:info-square-outline',
    idx: 1
  },
  {
    title: 'Cpu',
    icon: 'i-solar:cpu-outline',
    idx: 2
  },
  {
    title: 'Serial',
    icon: 'i-solar:printer-minimalistic-outline',
    idx: 3
  }
]
</script>

<style scoped lang="scss">
.no-scroll-bar {
  scrollbar-width: none;
}
</style>
