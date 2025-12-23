import type { User } from '@supabase/supabase-js';

import { defineStore } from 'pinia';
import { ref } from 'vue';

import { supabase } from '@/services/supabase';

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null);
  const loading = ref(false);

  // Initialize: Check for existing session
  async function init() {
    const { data } = await supabase.auth.getUser();
    user.value = data.user;
  }

  async function login(email: string, pass: string) {
    try {
      loading.value = true;
      const { data, error } = await supabase.auth.signInWithPassword({
        email,
        password: pass,
      });
      if (error)
        throw error;
      user.value = data.user;
      return true;
    }
    catch (error) {
      console.error('Login Error:', error);
      throw error; // Propagate error to UI
    }
    finally {
      loading.value = false;
    }
  }

  async function logout() {
    await supabase.auth.signOut();
    user.value = null;
    window.location.reload(); // Force reload to clear all states
  }

  return { user, loading, init, login, logout };
});
