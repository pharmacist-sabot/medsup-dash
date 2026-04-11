<!-- src/components/dashboard/KpiCard.vue -->
<script setup lang="ts">
import type { Component } from 'vue';

defineProps<{
  title: string;
  value: string;
  subValue?: string;
  icon?: Component; // Lucide Icon Type
  colorClass?: string;
}>();
</script>

<template>
  <div class="p-6 bg-cream border border-block-gold shadow-golden hover:shadow-golden transition-all duration-200">
    <div class="flex items-center justify-between mb-4">
      <div class="w-12 h-12 flex items-center justify-center" :class="colorClass || 'bg-sunshine-700/20 text-[#fa520f]'">
        <component :is="icon" v-if="icon" class="w-6 h-6" />
      </div>
    </div>
    <div>
      <h3 class="text-sm text-[#1f1f1f]/60 mb-1">
        {{ title }}
      </h3>
      <div class="text-2xl text-[#1f1f1f]">
        {{ value }}
      </div>
      <p v-if="subValue" class="text-xs text-[#1f1f1f]/40 mt-1">
        {{ subValue }}
      </p>
    </div>
  </div>
</template>
