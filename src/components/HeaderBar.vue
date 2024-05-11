<template>
  <header class="bg-blue-3 text-white shadow-sm border-b-1 p-2 overflow-y-auto">
    <span class="flex gap-2 justify-center w-fit mx-auto text-3xl sm:text-5xl">
      <button class="i-pixelarticons:folder" v-tooltip="t('open')" />
      <button class="i-pixelarticons:save" v-tooltip="t('save')" />
      <span class="divider" />
      <button class="i-pixelarticons:play" v-tooltip="tooltipPlay" :disabled="disablePlay" />
      <button class="i-pixelarticons:pause" v-tooltip="t('pause')" :disabled="disablePause" />
      <button class="i-pixelarticons:next" v-tooltip="t('next')" :disabled="disableNext" />
      <button class="i-pixelarticons:reload" v-tooltip="t('reload')" />
      <button class="i-pixelarticons:close" v-tooltip="t('shutdown')" :disabled="disableShutdown" />
      <span class="divider" />
      <Menu class="size-7.5 sm:size-12 relative">
        <button :class="[speedIcon, 'size-full absolute inset-block-0']" @dblclick="resetSpeed" />
        <template #popper>
          <div class="p-2 md:p-4 flex content-center">
            <label class="me-2 md:me-4 text-xs">
              X{{ speedScale.toFixed(2).padStart(5, ' ') }}
            </label>
            <MySlider v-model="loggedSpeedScale" :min="-4" :max="4" :step="0.1" />
          </div>
        </template>
      </Menu>
      <button class="i-pixelarticons:volume-2" v-tooltip="t('volume')" />
      <button class="i-pixelarticons:camera" v-tooltip="t('shot')" />
      <button class="i-pixelarticons:more-horizontal" v-tooltip="t('more')" />
    </span>
  </header>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { EmulatorState } from '@/emulator/state'
import { computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { Menu } from 'floating-vue'
import MySlider from './MySlider.vue'

const { t } = useI18n({
  inheritLocale: true,
  useScope: 'local'
})

const emu = useEmulator()
const state = emu.useState()
const speedScale = emu.useSpeedScale()
const volume = emu.useVolume()

const tooltipPlay = computed(() =>
  state.value === EmulatorState.Paused ? t('resume') : t('start')
)

const disablePlay = computed(
  () => state.value === EmulatorState.Running || state.value === EmulatorState.Aborted
)

const disablePause = computed(() => state.value !== EmulatorState.Running)
const disableNext = computed(() => state.value !== EmulatorState.Paused)
const disableShutdown = computed(() => state.value === EmulatorState.Shutdown)

const loggedSpeedScale = computed({
  get: () => Math.log2(speedScale.value),
  set: (val) => (speedScale.value = 2 ** val)
})
const speedIcon = computed(() => {
  const val = speedScale.value
  if (val < 0.25) return 'i-pixelarticons:speed-slow'
  else if (val > 4) return 'i-pixelarticons:speed-fast'
  else return 'i-pixelarticons:speed-medium'
})
const resetSpeed = () => (speedScale.value = 1)
</script>

<script lang="ts">
const volumeIcons = [] as const
</script>

<style scoped lang="scss">
button {
  transition: color 0.2s ease-in-out;
  &:hover,
  &:focus {
    background-color: rgb(219, 234, 254);
  }
  &:disabled {
    background-color: rgb(229, 231, 235);
    pointer-events: none;
  }
}

.divider {
  border-left: 2px solid rgba(#fff, 0.5);
  margin-block: 0.25rem;
  margin-inline: 0.5rem;
  @media (min-width: 768px) {
    & {
      margin-inline: 1rem;
    }
  }
}
</style>

<i18n>
{
  "cn": {
    "open": "打开",
    "save": "保存",
    "resume": "继续",
    "start": "开始",
    "pause": "暂停",
    "restart": "重启",
    "shutdown": "关闭",
    "speed": "速度",
    "volume": "音量",
    "shot": "截图",
    "more": "更多"
  },
  "en": {
    "open": "Open",
    "save": "Save",
    "resume": "Resume",
    "start": "Start",
    "pause": "Pause",
    "restart": "Restart",
    "shutdown": "Shutdown",
    "speed": "Speed",
    "volume": "Volume",
    "shot": "Shot",
    "more": "More"
  }
}
</i18n>