<!-- src/components/common/BaseButton.vue -->
<script setup lang="ts">
type Props = {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
};

withDefaults(defineProps<Props>(), {
  variant: 'primary',
});

const variantClasses = {
  primary:
        'bg-[#1f1f1f] text-white hover:bg-[#fa520f] shadow-golden-sm hover:shadow-golden',
  secondary:
        'bg-[#fff0c2] text-[#1f1f1f] border border-[#ffe295] hover:bg-[#ffd06a] hover:border-[#ffa110]',
  danger:
        'bg-[#fa520f] text-white hover:bg-[#fb6424] shadow-golden-sm',
  ghost:
        'bg-transparent text-[#1f1f1f]/60 hover:bg-[oklab(0_0_0/0.06)] hover:text-[#1f1f1f]',
};
</script>

<template>
  <button
    class="inline-flex items-center justify-center px-3 py-3 rounded-none font-normal tracking-wide transition-all duration-200 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-[#fa520f] disabled:opacity-50 disabled:cursor-not-allowed uppercase text-sm"
    :class="variantClasses[variant]"
  >
    <slot />
  </button>
</template>
