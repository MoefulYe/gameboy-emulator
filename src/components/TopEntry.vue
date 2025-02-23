<template>
  <div class="min-h-screen max-h-screen bg-coolgray-1">
    <HeaderBar />
    <div class="content">
      <GameBoy class="inline-block grow" />
      <Transition name="sidebar">
        <SideBar
          v-show="sidebarShow"
          v-resizable="ASIDE_RESIZABLE_CONFIG"
          ref="sidebar"
          class="inline-block w-1/3"
        />
      </Transition>
    </div>
    <Teleport to="body">
      <Transition name="popup">
        <Popup v-show="popupShow" @close="popupShow = false" ref="popup" />
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import 'vue-resizables/style'
import { provide, ref, useTemplateRef, watch } from 'vue'
import { createEmulator, emuKey } from '@/emulator'
import { vResizable, type ResizableConfig } from 'vue-resizables'
import HeaderBar from './HeaderBar.vue'
import GameBoy from './GameBoy.vue'
import SideBar, { sideBarShowKey } from './SideBar'
import Popup, { popupShowKey } from './Popup'
import { useDocumentListener } from '@/utils/hooks'

const popup = useTemplateRef('popup')
const emu = await createEmulator()
provide(emuKey, emu)
const sidebarShow = ref(false)
const popupShow = ref(false)
provide(sideBarShowKey, sidebarShow)
provide(popupShowKey, popupShow)

watch(popupShow, (show) => {
  if (show) {
    popup.value?.update()
  }
})

useDocumentListener('keyup', (e) => {
  e.stopImmediatePropagation()
  switch (e.key) {
    case 'a':
      popupShow.value = !popupShow.value
      break
    case 's':
      sidebarShow.value = !sidebarShow.value
      break
  }
})
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
.content {
  display: flex;
  height: calc(100vh - 47px);
  @media (min-width: 640px) {
    height: calc(100vh - 65px);
  }
}

.sidebar-enter-from,
.sidebar-leave-to {
  transform: translateX(100%);
}

.sidebar-enter-active,
.sidebar-leave-active {
  transition: transform 300ms ease-out;
}

.popup-enter-active {
  animation: fade-in 200ms;
}
.popup-leave-active {
  animation: fade-in 200ms reverse;
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
