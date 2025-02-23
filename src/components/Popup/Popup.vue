<template>
  <div class="--popup font-sans">
    <div
      ref="content"
      class="bg-coolgray-50 w-5/6 h-11/12 max-w-4xl max-h-6xl rounded-lg shadow-lg p-4 overflow-y-auto"
    >
      <div class="flex flex-col">
        <div class="-m-1.5 overflow-x-auto">
          <div class="p-1.5 min-w-full inline-block align-middle">
            <div class="overflow-hidden">
              <table class="min-w-full divide-y divide-gray-200 dark:divide-neutral-700">
                <thead>
                  <tr>
                    <th
                      scope="col"
                      class="px-6 py-3 text-start text-xs font-medium text-gray-500 uppercase dark:text-neutral-500"
                    >
                      game
                    </th>
                    <th
                      scope="col"
                      class="px-6 py-3 text-start text-xs font-medium text-gray-500 uppercase dark:text-neutral-500"
                    >
                      create at
                    </th>
                    <th
                      scope="col"
                      class="px-6 py-3 text-start text-xs font-medium text-gray-500 uppercase dark:text-neutral-500"
                    >
                      last access
                    </th>
                    <th
                      scope="col"
                      class="px-6 py-3 text-end text-xs font-medium text-gray-500 uppercase dark:text-neutral-500"
                    >
                      Action
                    </th>
                  </tr>
                </thead>
                <tbody class="divide-y divide-gray-200 dark:divide-neutral-700">
                  <tr v-for="save of saves" :key="save.id">
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-800 dark:text-neutral-200"
                    >
                      {{ save.metadata.cartTitle }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-800 dark:text-neutral-200"
                    >
                      {{
                        save.metadata.createdAt !== undefined
                          ? formatYYYYMMDDHHMMSS(save.metadata.createdAt)
                          : 'None'
                      }}
                    </td>
                    <td
                      class="px-6 py-4 whitespace-nowrap text-sm text-gray-800 dark:text-neutral-200"
                    >
                      {{
                        save.metadata.lastAccessed !== undefined
                          ? formatYYYYMMDDHHMMSS(save.metadata.lastAccessed)
                          : 'None'
                      }}
                    </td>
                    <td class="px-6 py-4 whitespace-nowrap text-end text-sm font-medium">
                      <button
                        type="button"
                        class="inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent text-blue-600 hover:text-blue-800 focus:outline-none focus:text-blue-800 disabled:opacity-50 disabled:pointer-events-none dark:text-blue-500 dark:hover:text-blue-400 dark:focus:text-blue-400 mr-4"
                        @click.stop="del(save.id)"
                      >
                        Delete
                      </button>
                      <button
                        type="button"
                        class="inline-flex items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent text-blue-600 hover:text-blue-800 focus:outline-none focus:text-blue-800 disabled:opacity-50 disabled:pointer-events-none dark:text-blue-500 dark:hover:text-blue-400 dark:focus:text-blue-400"
                        @click.stop="emu.load(save)"
                      >
                        Load
                      </button>
                    </td>
                  </tr>
                </tbody>
              </table>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import type { Save } from '@/emulator/constants'
import { formatYYYYMMDDHHMMSS } from '@/utils/date'
import { useWindowListener } from '@/utils/hooks'
import { shallowRef, triggerRef, useTemplateRef } from 'vue'
const emit = defineEmits<Emits>()
const emu = useEmulator()
const content = useTemplateRef('content')
useWindowListener('click', (e) => {
  const ok = content.value!.contains(e.target as any)
  if (!ok) {
    emit('close')
  }
})

const del = (id: number) => {
  const idx = saves.value.findIndex((v) => v.id === id)
  saves.value.splice(idx, 1)
  emu.db.delete('saves', id)
  triggerRef(saves)
}
const saves = shallowRef<Save[]>([])
const update = async () => {
  saves.value = await emu.db.getAll('saves')
}

defineExpose({ update })
</script>

<script lang="ts">
type Emits = {
  close: []
}
</script>

<style lang="scss" scoped>
.--popup {
  z-index: 10;
  position: absolute;
  inset: 0;
  backdrop-filter: brightness(80%) blur(2px);
  display: flex;
  align-items: center;
  justify-content: center;
}
</style>
