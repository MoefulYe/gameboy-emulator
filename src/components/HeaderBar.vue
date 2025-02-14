<template>
  <header id="header-bar" class="bg-blue-3 text-white shadow-sm border-b-1 p-2 overflow-y-auto">
    <span class="flex gap-2 justify-center w-fit mx-auto text-3xl sm:text-5xl">
      <button class="i-pixelarticons:folder" v-tooltip="'Open'" @click="openRom" />
      <button class="i-pixelarticons:save" v-tooltip="'save'" />
      <span class="divider" />
      <button
        class="i-pixelarticons:play"
        v-tooltip="tooltipPlay"
        :disabled="disablePlay"
        @click="emu.start()"
      />
      <button class="i-pixelarticons:pause" v-tooltip="'pause'" :disabled="disablePause" />
      <button class="i-pixelarticons:next" v-tooltip="'next'" :disabled="disableNext" />
      <button class="i-pixelarticons:reload" v-tooltip="'reset'" />
      <button class="i-pixelarticons:close" v-tooltip="'shutdown'" :disabled="disableShutdown" />
      <span class="divider" />
      <Menu class="size-7.5 sm:size-12 relative">
        <button :class="[speedIcon, 'size-full absolute inset-block-0']" @dblclick="resetSpeed" />
        <template #popper>
          <div class="p-2 md:p-4 flex content-center">
            <label class="me-2 md:me-4 text-(xs coolgray-6)">
              {{ speedStr }}
            </label>
            <MySlider v-model="loggedSpeedScale" :min="-4" :max="4" :step="0.1" />
          </div>
        </template>
      </Menu>
      <Menu class="size-7.5 sm:size-12 relative">
        <button :class="[volumeIcon, 'size-full absolute inset-block-0']" @dblclick="resetVolume" />
        <template #popper>
          <div class="p-2 md:p-4 flex content-center">
            <label class="me-2 md:me-4 text-(xs coolgray-6) whitespace-pre">
              {{ volumeStr }}
            </label>
            <MySlider v-model="volume" :min="0" :max="150" />
          </div>
        </template>
      </Menu>
      <button
        class="i-pixelarticons:debug hidden md:block"
        v-tooltip="'Debug'"
        @click="toggleSideBar"
      />
      <button
        class="i-pixelarticons:more-horizontal"
        v-tooltip="'More'"
        @click.stop="popupShow = true"
      />
    </span>
  </header>
</template>

<script setup lang="ts">
import { useEmulator } from '@/emulator'
import { State } from '@/emulator/constants'
import { computed } from 'vue'
import { Menu } from 'floating-vue'
import MySlider from './common/MySlider.vue'
import { openFile } from '@/utils/fs'
import { useShowSideBar } from './SideBar'
import { useShowPopup } from './Popup'
const emu = useEmulator()
const sideBarShow = useShowSideBar()
const popupShow = useShowPopup()
const { freqScale, volume } = emu.config
const { state } = emu.stat

const tooltipPlay = computed(() => (state.value === State.Paused ? 'resume' : 'start'))

const disablePlay = computed(() => {
  const s = state.value
  return s === State.Running || s === State.Aborted
})
const disablePause = computed(() => state.value !== State.Running)
const disableNext = computed(() => state.value !== State.Paused)
const disableShutdown = computed(() => state.value === State.Shutdown)

const loggedSpeedScale = computed({
  get: () => Math.log2(freqScale.value),
  set: (val) => (freqScale.value = 2 ** val)
})
const speedIcon = computed(() => {
  const val = freqScale.value
  if (val < 0.25) return 'i-pixelarticons:speed-slow'
  else if (val > 4) return 'i-pixelarticons:speed-fast'
  else return 'i-pixelarticons:speed-medium'
})
const resetSpeed = () => (freqScale.value = 1)
const speedStr = computed(() => `X${freqScale.value.toFixed(2).padStart(5, ' ')}`)

const volumeIcon = computed(() => {
  const _val = Math.floor((volume.value + 49) / 50)
  const val = _val > 3 ? 3 : _val
  return volumeIcons[val]
})
const resetVolume = () => {
  if (volume.value !== 0) {
    volume.value = 0
  } else {
    volume.value = 50
  }
}
const volumeStr = computed(() => `${volume.value.toString().padStart(3)}%`)

const toggleSideBar = () => (sideBarShow.value = !sideBarShow.value)

const openRom = async () => {
  const file = await openFile()
  const buf = await file.arrayBuffer()
  const rom = new Uint8Array(buf)
  emu.openRom(rom)
}
</script>

<script lang="ts">
const volumeIcons = [
  'i-pixelarticons:volume-x',
  'i-pixelarticons:volume-1',
  'i-pixelarticons:volume-2',
  'i-pixelarticons:volume-3'
] as const
</script>

<style scoped lang="scss">
button {
  transition: color 0.2s ease-in-out;
  &:hover {
    background-color: rgb(219, 234, 254);
  }
  &:disabled {
    background-color: rgb(229, 231, 235);
    pointer-events: none;
  }
}

.divider {
  border-left: 2px solid rgb(219, 234, 254);
  margin-block: 0.25rem;
  margin-inline: 0.5rem;
}

#header-bar {
  scrollbar-width: none;
}
</style>
