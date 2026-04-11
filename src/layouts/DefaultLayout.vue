<!-- src/layouts/DefaultLayout.vue -->
<script setup lang="ts">
import AppNavbar from '@/components/common/AppNavbar.vue';
</script>

<template>
  <div class="flex flex-col min-h-screen bg-warm-ivory">
    <AppNavbar />

    <main class="grow container mx-auto px-4 py-8 max-w-6xl">
      <slot />
    </main>

    <footer class="bg-[#1f1f1f] mt-auto">
      <!-- Mistral block gradient accent bar -->
      <div
        class="h-1 w-full"
        style="background: linear-gradient(90deg, #ffd900, #ffe295, #ffa110, #ff8105, #fb6424, #fa520f)"
      />
      <div class="container mx-auto px-4 py-6 text-center text-white/50 text-sm">
        &copy; {{ new Date().getFullYear() }} Medical Support Dashboard. Sabot Hospital.
      </div>
    </footer>
  </div>
</template>
