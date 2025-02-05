<template>
  <Transition name="fade">
    <Suspense>
      <TopEntry />
      <template #fallback>
        <SplashScreen />
      </template>
    </Suspense>
  </Transition>
</template>

<script setup lang="ts">
import TopEntry from '@/components/TopEntry.vue'
import SplashScreen from '@/components/SplashScreen.vue'
</script>

<style lang="scss" scoped>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.5s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
