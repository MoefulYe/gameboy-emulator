<script setup lang="ts">
import { useElementClientHeight } from '@/utils/hooks'
import { ref, useTemplateRef } from 'vue'
const { title = '', open = false } = defineProps<{
  title?: string
  open?: boolean
}>()
defineSlots<{
  default(props: {}): any
}>()
const dropdown = ref(open)
const dropdownContent = useTemplateRef('dropdownContent')
const contentHeight = useElementClientHeight(dropdownContent)
const toggle = () => (dropdown.value = !dropdown.value)
</script>

<template>
  <div class="dropdown-set-group b-(1 coolgray-75) rounded-md">
    <div class="dropdown-set-btn flex justify-between items-center" @click="toggle">
      <div class="text-lg font-semibold text-coolgray-6">{{ title }}</div>
      <div class="flex items-center">
        <span
          class="i-ant-design:down-outlined expand-more animation size-6 text-coolgray-5"
          :class="{ active: dropdown }"
        ></span>
      </div>
    </div>
    <div
      class="dropdown-set-content-area animation border-t-1 b-coolgray-75"
      :class="{ active: dropdown }"
      :style="{ 'max-height': !dropdown ? 0 : `${contentHeight}px` }"
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
