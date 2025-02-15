<script setup lang="ts">
import { useElementClientHeight } from '@/utils/hooks'
import { useTemplateRef } from 'vue'
const { title = '' } = defineProps<{
  title?: string
}>()
defineSlots<{
  default(props: {}): any
}>()
const open = defineModel<boolean>('open', {
  default: false
})
const dropdownContent = useTemplateRef('dropdownContent')
const contentHeight = useElementClientHeight(dropdownContent)
const toggle = () => (open.value = !open.value)
</script>

<template>
  <div class="dropdown-set-group b-(1 coolgray-2) rounded-md">
    <div class="dropdown-set-btn flex justify-between items-center" @click="toggle">
      <div>{{ title }}</div>
      <div class="flex items-center">
        <span
          class="i-ant-design:down-outlined expand-more animation size-6 text-coolgray-5"
          :class="{ active: open }"
        ></span>
      </div>
    </div>
    <div
      class="dropdown-set-content-area animation border-t-1 b-coolgray-2"
      :class="{ active: open }"
      :style="{ 'max-height': !open ? 0 : `${contentHeight}px` }"
    >
      <div class="dropdown-set-content" ref="dropdownContent">
        <div class="content">
          <slot name="default"></slot>
        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
.animation {
  transition: 0.5s;
}
.dropdown-set-group {
  background: rgb(249, 250, 251);
  margin-top: 5px;
  .dropdown-set-btn {
    padding: 10px 15px;
    cursor: pointer;
    .expand-more {
      &.active {
        transform: rotate(180deg);
      }
    }
  }
  .dropdown-set-content-area {
    overflow: hidden;
    visibility: hidden;
    &.active {
      visibility: visible;
    }
    .dropdown-set-content {
      padding: 10px;
    }
    .content {
      background: rgb(249, 250, 251);
      padding: 1px;
    }
  }
}
</style>
