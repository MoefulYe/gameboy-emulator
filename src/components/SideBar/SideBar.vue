<script setup lang="ts">
import MyCollapse from '@/components/common/MyCollapse.vue'
import SummaryDisplay from './BasicDisplay.vue'
import LogDisplay from './LogDisplay'
import RomDisplay from './RomDisplay.vue'
</script>

<template>
  <aside
    class="bg-coolgray-50 m-8 p-4 rounded-lg font-mono min-w-36 text-coolgray-7 b-(1 coolgray-2) text-lg"
  >
    <MyCollapse title="Basic" open>
      <SummaryDisplay />
    </MyCollapse>
    <MyCollapse title="Rom">
      <RomDisplay />
    </MyCollapse>
    <MyCollapse title="Log" open>
      <LogDisplay />
    </MyCollapse>
  </aside>
</template>
