<template>
  <Transition name="fade">
    <Suspense>
      <TopEntry :delay="1000" />
      <template #fallback>
        <EmulatorLoading />
      </template>
    </Suspense>
  </Transition>
</template>

<script setup lang="ts">
import EmulatorLoading from '@/components/EmulatorLoading.vue'
import TopEntry from '@/components/TopEntry.vue'
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
