<!-- src/views/auth/LoginView.vue -->
<script setup lang="ts">
import { AlertCircle } from 'lucide-vue-next';
import { ref } from 'vue';
import { useRouter } from 'vue-router';

import BaseButton from '@/components/common/BaseButton.vue';
import { useAuthStore } from '@/stores/auth';

const auth = useAuthStore();
const router = useRouter();

const email = ref('');
const password = ref('');
const errorMsg = ref('');

async function handleLogin() {
  errorMsg.value = '';
  try {
    await auth.login(email.value, password.value);
    router.push('/');
  }
  catch (e: any) {
    errorMsg.value = e.message || 'Login failed';
  }
}
</script>

<template>
  <div class="w-full max-w-md mx-auto p-6">
    <div class="bg-white rounded-2xl shadow-xl border border-gray-100 p-8">
      <div class="text-center mb-8">
        <h1 class="text-2xl font-bold text-gray-900">
          Welcome Back
        </h1>
        <p class="text-gray-500 text-sm mt-2">
          Sign in to access the dashboard
        </p>
      </div>

      <form class="space-y-6" @submit.prevent="handleLogin">
        <div v-if="errorMsg" class="p-3 rounded-lg bg-red-50 text-red-600 text-sm flex items-center gap-2">
          <AlertCircle class="w-4 h-4" />
          {{ errorMsg }}
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Email</label>
          <input
            v-model="email"
            type="email"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-all"
            placeholder="pharmacist@sabot.hospital"
          >
        </div>

        <div>
          <label class="block text-sm font-medium text-gray-700 mb-1">Password</label>
          <input
            v-model="password"
            type="password"
            required
            class="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-indigo-500 focus:border-indigo-500 outline-none transition-all"
            placeholder="••••••••"
          >
        </div>

        <BaseButton
          type="submit"
          class="w-full justify-center"
          :disabled="auth.loading"
        >
          {{ auth.loading ? 'Signing in...' : 'Sign In' }}
        </BaseButton>
      </form>
    </div>
  </div>
</template>
