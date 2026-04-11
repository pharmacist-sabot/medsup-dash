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
  <div class="w-full max-w-md mx-auto px-4">
    <!-- Mistral block gradient accent bar -->
    <div
      class="h-1 w-full mb-0"
      style="background: linear-gradient(90deg, #ffd900, #ffe295, #ffa110, #ff8105, #fb6424, #fa520f);"
    />

    <!-- Card -->
    <div
      class="bg-cream border border-block-gold p-8"
      style="box-shadow: rgba(127,99,21,0.12) -8px 16px 39px, rgba(127,99,21,0.10) -33px 64px 72px, rgba(127,99,21,0.06) -73px 144px 97px;"
    >
      <!-- Header -->
      <div class="mb-8">
        <!-- Logo mark -->
        <div class="flex items-center gap-3 mb-6">
          <div
            class="w-9 h-9 flex items-center justify-center"
            style="background: linear-gradient(135deg, #ffa110, #fa520f);"
          >
            <svg
              class="w-5 h-5 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor"
              stroke-width="1.5"
            >
              <path
                stroke-linecap="square" stroke-linejoin="miter"
                d="M9 12h6m-3-3v6M4.5 19.5l15-15M4.5 4.5l15 15"
              />
            </svg>
          </div>
          <span class="text-[#1f1f1f] text-base tracking-wide">MedValue Support</span>
        </div>

        <h1 class="text-2xl text-[#1f1f1f] leading-tight">
          Welcome Back
        </h1>
        <p class="text-[#1f1f1f]/50 text-sm mt-1">
          Sign in to access the dashboard
        </p>
      </div>

      <form class="space-y-5" @submit.prevent="handleLogin">
        <!-- Error message -->
        <div
          v-if="errorMsg"
          class="p-3 bg-warm-ivory border border-[#fa520f]/30 text-[#fa520f] text-sm flex items-center gap-2"
        >
          <AlertCircle class="w-4 h-4 shrink-0" />
          {{ errorMsg }}
        </div>

        <!-- Email -->
        <div>
          <label class="block text-xs text-[#1f1f1f]/60 mb-1.5 uppercase tracking-wider">
            Email
          </label>
          <input
            v-model="email" type="email" required
            class="w-full px-4 py-3 bg-warm-ivory border border-block-gold rounded-none text-[#1f1f1f] text-sm placeholder:text-[#1f1f1f]/30 outline-none transition-all duration-200 focus:border-[#fa520f] focus:ring-1 focus:ring-[#fa520f]"
            placeholder="pharmacist@sabot.hospital"
          >
        </div>

        <!-- Password -->
        <div>
          <label class="block text-xs text-[#1f1f1f]/60 mb-1.5 uppercase tracking-wider">
            Password
          </label>
          <input
            v-model="password" type="password" required
            class="w-full px-4 py-3 bg-warm-ivory border border-block-gold rounded-none text-[#1f1f1f] text-sm placeholder:text-[#1f1f1f]/30 outline-none transition-all duration-200 focus:border-[#fa520f] focus:ring-1 focus:ring-[#fa520f]"
            placeholder="••••••••"
          >
        </div>

        <!-- Submit -->
        <div class="pt-2">
          <BaseButton type="submit" class="w-full justify-center" :disabled="auth.loading">
            {{ auth.loading ? 'Signing in...' : 'Sign In' }}
          </BaseButton>
        </div>
      </form>
    </div>

    <!-- Bottom caption -->
    <p class="text-center text-xs text-[#1f1f1f]/30 mt-4">
      Medical Support Dashboard &mdash; Sabot Hospital
    </p>
  </div>
</template>
