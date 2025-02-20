<template>
  <div>
    <label>input-mode: </label>
    <span @dblclick="toggleMode">
      {{ INPUT_MODE_STR[mode] }}
    </span>
  </div>
  <div>
    <label>gamepad-id: </label>
    <span>
      {{ gamepadId }}
    </span>
  </div>
  <div>
    <label>mappings: </label>
  </div>
  <div class="py-2 px-4 b-1 b-coolgray-2 rounded-sm text-sm flex flex-col gap-1">
    <div v-for="(m, i) of mapping" :key="i" class="flex gap-2 items-center">
      <label>button{{ i }}: </label>
      <MyDropdown
        :model-value="m"
        :items="DROPDOWN_ITEMS"
        :btn-class="DROPDOWN_BTN_CLASS"
        :list-class="DROPDOWN_LIST_CLASS"
        :item-class="DROPDOWN_ITEM_CLASS"
        @update:model-value="(v) => updateMapping(i, v)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import {
  GameboyLayoutButton,
  GamepadMode,
  INPUT_MODE_STR
} from '@/emulator/input/gamepad/constants'
import MyDropdown from '../common/MyDropdown.vue'
import type { StandardButton } from '@/emulator/input/gamepad/PhysicalGamepad'

const gamepad = useEmulator().gamepad
const { mode, physical } = gamepad
const { gamepadId, mapping } = physical

const toggleMode = () => {
  switch (mode.value) {
    case GamepadMode.Physical:
      mode.value = GamepadMode.Virtual
      break
    case GamepadMode.Virtual:
      mode.value = GamepadMode.Physical
      break
  }
}
const updateMapping = (idx: StandardButton, to: GameboyLayoutButton) => {
  const ret = [...(mapping.value as any as GameboyLayoutButton[])]
  ret[idx] = to
  mapping.value = ret as any
}
</script>

<script lang="ts">
const DROPDOWN_ITEMS = [
  { label: 'right', key: GameboyLayoutButton.Right },
  { label: 'left', key: GameboyLayoutButton.Left },
  { label: 'up', key: GameboyLayoutButton.Up },
  { label: 'down', key: GameboyLayoutButton.Down },
  { label: 'a', key: GameboyLayoutButton.A },
  { label: 'b', key: GameboyLayoutButton.B },
  { label: 'start', key: GameboyLayoutButton.Start },
  { label: 'select', key: GameboyLayoutButton.Select },
  { label: 'none', key: GameboyLayoutButton.None }
] as const
const DROPDOWN_BTN_CLASS = 'hover:bg-coolgray-2 b-1 b-coolgray-2'
const DROPDOWN_LIST_CLASS = 'b-(1 coolgray-2) bg-coolgray-50'
const DROPDOWN_ITEM_CLASS = 'hover:bg-coolgray-2'
</script>
