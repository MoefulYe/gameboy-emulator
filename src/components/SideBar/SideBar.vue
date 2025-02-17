<script setup lang="ts">
import MyCollapse from '@/components/common/MyCollapse.vue'
import SummaryDisplay from './BasicDisplay.vue'
import LogDisplay from './LogDisplay'
import RomDisplay from './RomDisplay.vue'
import GamepadDisplay from './GamepadDisplay.vue'
import SerialDisplay from './SerialDisplay.vue'
import CPUStateDisplay from './CPUStateDisplay.vue'
import { useLocalStorage } from '@/utils/localstorage'
import TilesDisplay from './TilesDisplay.vue'

const basicOpen = useLocalStorage<boolean>('sidebar-basic', true)
const cpuOpen = useLocalStorage<boolean>('sidebar-cpu', true)
const romOpen = useLocalStorage<boolean>('sidebar-rom', false)
const gamepadOpen = useLocalStorage<boolean>('sidebar-gamepad', false)
const serialOpen = useLocalStorage<boolean>('sidebar-serial', false)
const logOpen = useLocalStorage<boolean>('sidebar-log', true)
const tilesOpen = useLocalStorage<boolean>('sidebar-tiles', false)
</script>

<template>
  <aside
    class="bg-coolgray-50 m-8 p-4 rounded-md min-w-36 text-coolgray-7 b-(1 coolgray-2) text-lg max-h-full overflow-auto no-scroller shadow-sm"
  >
    <MyCollapse title="Basic" v-model:open="basicOpen">
      <SummaryDisplay />
    </MyCollapse>
    <MyCollapse title="Cpu" v-model:open="cpuOpen">
      <CPUStateDisplay />
    </MyCollapse>
    <MyCollapse title="Rom" v-model:open="romOpen">
      <RomDisplay />
    </MyCollapse>
    <MyCollapse title="Gamepad" v-model:open="gamepadOpen">
      <GamepadDisplay />
    </MyCollapse>
    <MyCollapse title="Serial" v-model:open="serialOpen">
      <SerialDisplay />
    </MyCollapse>
    <MyCollapse title="Tiles" v-model:open="tilesOpen">
      <TilesDisplay />
    </MyCollapse>
    <MyCollapse title="Log" v-model:open="logOpen">
      <LogDisplay />
    </MyCollapse>
  </aside>
</template>
