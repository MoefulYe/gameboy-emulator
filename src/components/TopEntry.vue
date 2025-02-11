<template>
  <div class="flex flex-col h-screen bg-coolgray-1">
    <HeaderBar />
    <div class="flex grow">
      <EmulatorMain class="inline-block grow" />
      <Transition name="sidebar">
        <SideBar
          v-if="sidebarShow"
          v-resizable="ASIDE_RESIZABLE_CONFIG"
          ref="sidebar"
          class="inline-block w-1/3"
        />
      </Transition>
    </div>
    <Teleport to="body">
      <Transition name="popup">
        <Popup v-if="popupShow" @close="popupShow = false" />
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import 'vue-resizables/style'
import { provide, ref } from 'vue'
import { createEmulator, emuKey } from '@/emulator'
import { vResizable, type ResizableConfig } from 'vue-resizables'
import HeaderBar from './HeaderBar.vue'
import EmulatorMain from './EmulatorMain.vue'
import SideBar, { sideBarShowKey } from './SideBar'
import Popup, { popupShowKey } from './Popup'

const emu = await createEmulator()
const sidebarShow = ref(true)
const popupShow = ref(false)
provide(emuKey, emu)
provide(sideBarShowKey, sidebarShow)
provide(popupShowKey, popupShow)
</script>

<script lang="ts">
const ASIDE_RESIZABLE_CONFIG = {
  edge: {
    left: true
  },
  border: true,
  size: {}
} as const satisfies ResizableConfig
</script>

<style lang="scss" scoped>
.sidebar-enter-from,
.sidebar-leave-to {
  transform: translateX(100%);
}

.sidebar-enter-active,
.sidebar-leave-active {
  transition: transform 300ms ease-out;
}

.popup-enter-active {
  animation: fade-in 300ms;
}
.popup-leave-active {
  animation: fade-in 300ms reverse;
}

@keyframes fade-in {
  from {
    backdrop-filter: brightness(100%) blur(0px);
    opacity: 0%;
  }

  25% {
    backdrop-filter: brightness(90%) blur(1px);
    opacity: 0%;
  }

  50% {
    backdrop-filter: brightness(85%) blur(1px);
    opacity: 25%;
  }

  75% {
    backdrop-filter: brightness(80%) blur(2px);
    opacity: 60%;
  }

  to {
    backdrop-filter: brightness(80%) blur(2px);
    opacity: 100%;
  }
}
</style>
