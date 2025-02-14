<script setup lang="ts">
import MyCollapse from '@/components/common/MyCollapse.vue'
import SummaryDisplay from './BasicDisplay.vue'
import LogDisplay from './LogDisplay'
import RomDisplay from './RomDisplay.vue'
import GamepadDisplay from './GamepadDisplay.vue'
import SerialDisplay from './SerialDisplay.vue'
import CPUStateDisplay from './CPUStateDisplay.vue'
</script>

<template>
  <aside
    class="bg-coolgray-50 m-8 p-4 rounded-md min-w-36 text-coolgray-7 b-(1 coolgray-2) text-lg max-h-full overflow-auto no-scroller"
  >
    <MyCollapse title="Basic" open>
      <SummaryDisplay />
    </MyCollapse>
    <MyCollapse title="Cpu">
      <CPUStateDisplay />
    </MyCollapse>
    <MyCollapse title="Rom">
      <RomDisplay />
    </MyCollapse>
    <MyCollapse title="Gamepad">
      <GamepadDisplay />
    </MyCollapse>
    <MyCollapse title="Serial">
      <SerialDisplay />
    </MyCollapse>
    <MyCollapse title="Log" open>
      <LogDisplay />
    </MyCollapse>
  </aside>
</template>
