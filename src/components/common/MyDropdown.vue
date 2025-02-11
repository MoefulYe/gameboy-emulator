<template>
  <div class="dropdown-wrapper relative">
    <button
      v-on:click="show = !show"
      :class="btnClass"
      class="text-coolgray-50 py-2 px-4 whitespace-no-wrap rounded transition duration-300"
    >
      {{ items[idx] }}
      <i
        class="i-ant-design:down-outline ml-2 transition-(~ transform duration-300)"
        :class="{
          'rotate-180': show
        }"
      ></i>
    </button>
    <Transition name="fade">
      <div
        :class="btnClass"
        class="dropdown-menu text-coolgray-50 mt-1 rounded absolute z-10 shadow-lg w-40 max-w-xs"
        v-if="show"
      >
        <ul class="list-none overflow-hidden rounded">
          <li
            v-for="(item, i) of items"
            :key="item"
            @click="idx = i"
            class="flex justify-between transition duration-300 items-center py-2 px-4"
            :class="btnClass"
          >
            <a>{{ item }}</a>
            <span v-show="idx === i" class="i-ant-design:check-outlined" />
          </li>
        </ul>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

type Item = string

type Items = readonly Item[]
defineProps<{
  items: Items
  btnClass: string
}>()

const idx = defineModel<number>({ required: true })

const show = ref(false)
</script>

<style lang="scss" scoped>
button {
  cursor: pointer;
  &:focus {
    outline: none;
  }
}

/* Animations */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s;
}
.fade-enter,
.fade-leave-to {
  opacity: 0;
}
</style>
