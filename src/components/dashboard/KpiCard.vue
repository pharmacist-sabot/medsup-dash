<!-- src/components/dashboard/KPICard.vue -->
<script setup lang="ts">
defineProps<{
  title: string;
  value: string;
  subValue?: string;
  icon?: any; // Lucide Icon Type
  colorClass?: string;
}>();
</script>

<template>
  <div class="p-6 bg-white rounded-xl border border-gray-100 shadow-sm hover:shadow-md transition-all duration-200">
    <div class="flex items-center justify-between mb-4">
      <div
        class="w-12 h-12 rounded-lg flex items-center justify-center"
        :class="colorClass || 'bg-blue-50 text-blue-600'"
      >
        <component :is="icon" v-if="icon" class="w-6 h-6" />
      </div>
    </div>
    <div>
      <h3 class="text-sm font-medium text-gray-500 mb-1">
        {{ title }}
      </h3>
      <div class="text-2xl font-bold text-gray-900">
        {{ value }}
      </div>
      <p v-if="subValue" class="text-xs text-gray-400 mt-1">
        {{ subValue }}
      </p>
    </div>
  </div>
</template>
