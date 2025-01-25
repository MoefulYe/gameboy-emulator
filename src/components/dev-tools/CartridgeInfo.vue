<template>
  <p v-if="cartridgeInfo !== undefined">
    <strong>Title</strong> {{ cartridgeInfo.title }}<br />
    <strong>CartType</strong> {{ cartridgeInfo.cartType }}<br />
    <strong>RomSize</strong> {{ cartridgeInfo.romSize }}<br />
    <strong>RamSize</strong> {{ cartridgeInfo.ramSize }}<br />
    <strong>Dest</strong> {{ cartridgeInfo.dest }}<br />
    <strong>Publisher</strong> {{ cartridgeInfo.publisher }}<br />
    <strong>Version</strong> {{ cartridgeInfo.version }}<br />
  </p>
  <p v-else>
    Nocart
  </p>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
const { cartridgeInfo } = useEmulator()
</script>