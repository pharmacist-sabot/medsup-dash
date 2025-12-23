<!-- src/layouts/DefaultLayout.vue -->
<script setup lang="ts">
import AppNavbar from '@/components/common/AppNavbar.vue';
</script>

<template>
  <div class="flex flex-col min-h-screen bg-slate-50">
    <AppNavbar />

    <main class="grow container mx-auto px-4 py-8 max-w-6xl">
      <slot />
    </main>

    <footer class="bg-white border-t border-gray-200 py-6 mt-auto">
      <div class="container mx-auto px-4 text-center text-gray-500 text-sm">
        &copy; {{ new Date().getFullYear() }} Medical Support Dashboard. Sabot Hospital.
      </div>
    </footer>
  </div>
</template>
