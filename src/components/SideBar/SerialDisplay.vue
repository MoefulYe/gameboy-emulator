<template>
  <div>
    <div></div>
    <label>output: </label>
    <div>
      {{ bytes !== '' ? bytes : 'None' }}
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useEmulator } from '@/emulator'

const bytes = useEmulator().stat.serialBytes
</script>
