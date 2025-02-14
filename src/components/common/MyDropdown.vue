<template>
  <div class="dropdown-wrapper relative">
    <button
      v-on:click="show = !show"
      class="py-2 px-4 whitespace-no-wrap rounded transition duration-300 flex flex-nowrap items-center"
      :class="btnClass"
    >
      <span>{{ items[idx].label }}</span>
      <i
        class="i-ant-design:down-outline ml-2 transition-(~ transform duration-300)"
        :class="{
          'rotate-180': show
        }"
      ></i>
    </button>
    <Transition name="fade">
      <div class="" v-show="show">
        <ul
          class="mt-1 rounded absolute z-10 w-40 max-w-xs list-none overflow-hidden rounded"
          :class="listClass"
        >
          <li
            v-for="({ label, key }, i) of items"
            :key="key"
            @click="
              () => {
                idx = i
                val = key
              }
            "
            class="flex flex-nowrap justify-between transition duration-300 items-center py-2 px-4 whitespace-nowrap"
            :class="itemClass"
          >
            <span>{{ label }}</span>
            <span v-show="idx === i" class="i-ant-design:check-outlined" />
          </li>
        </ul>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts" generic="Label extends string, Key extends string | number">
import { ref } from 'vue'

type Item = {
  label: Label
  key: Key
}
type Items = readonly Item[]

const { items } = defineProps<{
  items: Items
  btnClass: string
  listClass: string
  itemClass: string
}>()

const val = defineModel<Key>({ required: true })
const idx = ref(items.findIndex((item) => item.key === val.value))
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
