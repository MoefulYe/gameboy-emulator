<template>
  <ul class="p-1" v-if="metadata !== undefined">
    <li>
      <label>created-at: </label>
      <span>
        {{ metadata.createdAt !== undefined ? formatYYYYMMDDHHMMSS(metadata.createdAt) : 'None' }}
      </span>
    </li>
    <li>
      <label>last-access: </label>
      <span>
        {{
          metadata.lastAccessed !== undefined ? formatYYYYMMDDHHMMSS(metadata.lastAccessed) : 'None'
        }}
      </span>
    </li>
  </ul>
  <div v-else>load rom first</div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { formatYYYYMMDDHHMMSS } from '@/utils/date'

const metadata = useEmulator().stat.saveMetaData
</script>
