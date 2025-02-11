<template>
  <div class="flex gap-2">
    <MyDropdown
      :items="LOG_STR"
      v-model="logFilter"
      btn-class="bg-blue-3 hover:bg-blue-4"
      v-tooltip="'filter logs'"
    />
    <button
      class="b-1 b-coolgray-3 p-1 px-2 rounded-md"
      @click="logs.length = 0"
      v-tooltip="'clear'"
    >
      <span class="i-ant-design:close-outlined text-coolgray-5"></span>
    </button>
  </div>
  <VirtualList
    class="h-96 overflow-y-auto no-scroller m-2 b-1 b-coolgray-2 p-2 rounded-md text-base"
    data-key="id"
    ref="list"
    :data-sources="logs"
    :data-component="LogDisplayItem"
  />
</template>
<script setup lang="ts">
import { useTemplateRef, watch, type ShallowRef } from 'vue'
// @ts-ignore
import VirtualList from 'vue3-virtual-scroll-list'
import { logs, filter } from '@/emulator/logger'
import LogDisplayItem from './LogDisplayItem.vue'
import MyDropdown from '@/components/common/MyDropdown.vue'
import { LOG_STR } from '@/emulator/constants'

const list: Readonly<ShallowRef<any>> = useTemplateRef('list')
watch(logs, () => list.value?.scrollToBottom())
const logFilter = filter
</script>

<script lang="ts"></script>

<style lang="scss" scoped>
.no-scroller {
  scrollbar-width: none;
}
</style>
