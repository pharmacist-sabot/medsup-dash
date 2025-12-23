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
  <header class="bg-white border-b border-gray-200 sticky top-0 z-30">
    <div class="container mx-auto px-4 h-16 flex items-center justify-between">
      <!-- Logo -->
      <div class="flex items-center gap-2">
        <div class="bg-indigo-600 p-1.5 rounded-lg">
          <Activity class="w-5 h-5 text-white" />
        </div>
        <span class="font-bold text-gray-800 text-lg">MedValue Support</span>
      </div>

      <!-- User Menu -->
      <div v-if="authStore.user" class="flex items-center gap-4">
        <span class="text-sm text-gray-600 hidden md:block">
          {{ authStore.user.email }}
        </span>
        <button
          class="p-2 text-gray-500 hover:text-red-600 hover:bg-red-50 rounded-full transition-colors"
          title="Logout"
          @click="handleLogout"
        >
          <LogOut class="w-5 h-5" />
        </button>
      </div>
    </div>
  </header>
</template>
