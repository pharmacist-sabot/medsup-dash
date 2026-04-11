<!-- src/components/common/AppNavbar.vue -->
<script setup lang="ts">
import { Activity, LogOut } from 'lucide-vue-next';
import { useRouter } from 'vue-router';

import { useAuthStore } from '@/stores/auth';

const authStore = useAuthStore();
const router = useRouter();

async function handleLogout() {
  await authStore.logout();
  router.push('/login');
}
</script>

<template>
  <header class="bg-[#1f1f1f] sticky top-0 z-30">
    <!-- Mistral block gradient accent bar -->
    <div
      class="h-0.5 w-full"
      style="background: linear-gradient(90deg, #ffd900, #ffe295, #ffa110, #ff8105, #fb6424, #fa520f);"
    />

    <div class="container mx-auto px-4 h-14 flex items-center justify-between max-w-6xl">
      <!-- Logo -->
      <div class="flex items-center gap-3">
        <div
          class="w-8 h-8 flex items-center justify-center"
          style="background: linear-gradient(135deg, #ffa110, #fa520f);"
        >
          <Activity class="w-4 h-4 text-white" />
        </div>
        <span class="text-white text-base tracking-wide">MedValue Support</span>
      </div>

      <!-- User Menu -->
      <div v-if="authStore.user" class="flex items-center gap-4">
        <span class="text-white/50 text-sm hidden md:block">
          {{ authStore.user.email }}
        </span>
        <button
          class="p-2 text-white/50 hover:text-[#fa520f] transition-colors duration-200" title="Logout"
          @click="handleLogout"
        >
          <LogOut class="w-4 h-4" />
        </button>
      </div>
    </div>
  </header>
</template>
