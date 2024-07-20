<template>
  <div>
    <p>
      {{ serialOutput }}
    </p>
    <p>
      {{ ascii }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { computed } from 'vue'
const { serialOutput } = useEmulator()
const ascii = computed(() => String.fromCharCode(...serialOutput))
</script>
