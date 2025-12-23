// src/router/index.ts
import { createRouter, createWebHistory } from 'vue-router';

import { useAuthStore } from '@/stores/auth';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/login',
      name: 'login',
      component: () => import('@/views/auth/LoginView.vue'),
      meta: { layout: 'blank' },
    },
    {
      path: '/',
      name: 'dashboard',
      component: () => import('@/views/dashboard/OverviewView.vue'),
      meta: { layout: 'default', requiresAuth: true },
    },
    {
      path: '/:pathMatch(.*)*',
      name: 'not-found',
      component: () => import('@/views/NotFoundView.vue'),
      meta: { layout: 'blank' },
    },
  ],
});

// Navigation Guard
router.beforeEach(async (to, from, next) => {
  const authStore = useAuthStore();

  // Ensure user state is initialized
  if (!authStore.user) {
    await authStore.init();
  }

  const isAuthenticated = !!authStore.user;

  if (to.meta.requiresAuth && !isAuthenticated) {
    next('/login');
  }
  else if (to.name === 'login' && isAuthenticated) {
    next('/');
  }
  else {
    next();
  }
});

export default router;
