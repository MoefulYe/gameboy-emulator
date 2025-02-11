<template>
  <input
    type="range"
    class="--my-slider"
    :class="customClass"
    :min="min"
    :max="max"
    :step="step"
    :disabled="disabled"
    v-model="valueStr"
  />
</template>

<script setup lang="ts">
import { computed } from 'vue'

withDefaults(
  defineProps<{
    min?: number
    max?: number
    disabled?: boolean
    customClass?: string
    step?: number
  }>(),
  {
    disabled: false
  }
)

const value = defineModel<number>({
  default: 0
})
const valueStr = computed({
  get: () => value.value.toString(),
  set: (v: string) => (value.value = parseFloat(v))
})
</script>

<style lang="scss">
.--my-slider {
  background-color: transparent;
  cursor: pointer;
  appearance: none;

  &:disabled {
    opacity: 0.5;
    pointer-events: none;
  }

  &:focus {
    outline: none;
  }

  &::-moz-range-thumb,
  &::-webkit-slider-thumb {
    width: 0.625rem;
    height: 0.625rem;
    margin-block-start: -0.125rem;
    appearance: none;
    background: #fff;
    box-shadow: 0 0 0px 4px rgba(96, 165, 250, 0.4);
    border-width: 0;
    border-radius: 9999px;
    transition: all 0.15s ease-in-out;
  }

  &::-moz-range-track,
  &::-webkit-slider-runnable-track {
    width: 100%;
    height: 0.5rem;
    background-color: rgb(243 244 246);
    border-radius: 9999px;
  }
}
</style>
