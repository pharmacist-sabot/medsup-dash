<!-- src/views/NotFoundView.vue -->
<script setup lang="ts">
import { RouterLink } from 'vue-router';

import BaseButton from '@/components/common/BaseButton.vue';
</script>

<template>
  <div class="text-center px-4 py-16">
    <!-- Large 404 display -->
    <div class="text-[120px] leading-none text-block-gold select-none" style="letter-spacing: -4px;">
      404
    </div>

    <!-- Mistral block gradient accent bar -->
    <div class="h-1 w-24 mx-auto my-6" style="background: linear-gradient(90deg, #ffd900, #ffa110, #fa520f);" />

    <div class="space-y-4 max-w-md mx-auto">
      <h2 class="text-2xl text-[#1f1f1f]">
        Page Not Found
      </h2>
      <p class="text-sm text-[#1f1f1f]/50 leading-relaxed">
        Oops! The page you are looking for might have been removed,
        had its name changed, or is temporarily unavailable.
      </p>

      <div class="pt-4">
        <RouterLink v-slot="{ navigate }" to="/" custom>
          <BaseButton variant="primary" @click="navigate">
            Back to Home
          </BaseButton>
        </RouterLink>
      </div>
    </div>
  </div>
</template>
