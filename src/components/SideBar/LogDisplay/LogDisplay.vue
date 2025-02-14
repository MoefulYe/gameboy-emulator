<template>
  <div class="flex gap-2 p-1">
    <MyDropdown
      :items="LOG_FILTER_ITEMS"
      v-model:model-value="logFilter"
      btn-class="hover:bg-coolgray-2 b-1 b-coolgray-2"
      list-class="b-(1 coolgray-2) bg-coolgray-50"
      item-class="hover:bg-coolgray-2"
      v-tooltip="'filter logs'"
    />
    <button
      class="b-1 b-coolgray-2 p-1 px-2 rounded-md"
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
import { LogLevel } from '@/emulator/constants'
import { throttle } from '@/utils/throttle'

const list: Readonly<ShallowRef<any>> = useTemplateRef('list')
watch(
  logs,
  throttle(() => list.value?.scrollToBottom())
)
const logFilter = filter
</script>

<script lang="ts">
const LOG_FILTER_ITEMS = [
  { label: 'Off', key: LogLevel.Off },
  { label: 'Error', key: LogLevel.Error },
  { label: 'Warn', key: LogLevel.Warn },
  { label: 'Info', key: LogLevel.Info },
  { label: 'Debug', key: LogLevel.Debug }
] as const
</script>
