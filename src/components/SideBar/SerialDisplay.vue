<template>
  <div>
    <div></div>
    <label>output: </label>
    <div class="max-h-33vh overflow-y-auto p-2 b-1 rounded b-coolgray-2 mt-2">
      {{ output }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useEmulator } from '@/emulator'
import { computed } from 'vue'

const bytes = useEmulator().stat.serialBytes
const output = computed(() => String.fromCharCode(...bytes))
</script>
