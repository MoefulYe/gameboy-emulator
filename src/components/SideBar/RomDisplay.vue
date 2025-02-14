<template>
  <div class="p-1">
    <div v-if="rom === undefined">no rom inserted</div>
    <ul v-else>
      <li>
        <label>title: </label> <span>{{ rom.title }}</span>
      </li>
      <li>
        <label>rom-type: </label> <span>{{ rom.cartType }}</span>
      </li>
      <li>
        <label>rom-size: </label> <span>{{ rom.romSize }}</span>
      </li>
      <li>
        <label>ram-size: </label> <span>{{ rom.ramSize ?? 'None' }}</span>
      </li>
      <li>
        <label>dest: </label> <span>{{ rom.dest }}</span>
      </li>
      <li>
        <label>publisher: </label> <span>{{ rom.publisher }}</span>
      </li>
      <li>
        <label>version: </label> <span>{{ rom.version }}</span>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
const rom = useEmulator().stat.rom
</script>
