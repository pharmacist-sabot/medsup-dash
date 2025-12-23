// src/stores/transactions.ts
import { defineStore } from 'pinia';
import { computed, ref } from 'vue';

import type { MedTransaction } from '@/types/database.types';

import { supabase } from '@/services/supabase';

export const useTransactionStore = defineStore('transactions', () => {
  const transactions = ref<MedTransaction[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const selectedFiscalYear = ref<number>(new Date().getFullYear() + (new Date().getMonth() >= 9 ? 1 : 0)); // Auto Fiscal Year

  // --- Getters / Computed Metrics ---

  const totalValue = computed(() =>
    transactions.value.reduce((sum, t) => sum + (t.drug_value || 0), 0),
  );

  const totalCount = computed(() => transactions.value.length);

  const averageValue = computed(() =>
    totalCount.value > 0 ? totalValue.value / totalCount.value : 0,
  );

  const recentTransactions = computed(() =>
    [...transactions.value]
      .sort((a, b) => new Date(b.transaction_date).getTime() - new Date(a.transaction_date).getTime())
      .slice(0, 10),
  );

  // Chart Data: Group by Drug Type
  const chartData = computed(() => {
    const grouped: Record<string, number> = {};
    transactions.value.forEach((t) => {
      const type = t.drug_type || 'Unknown';
      grouped[type] = (grouped[type] || 0) + t.drug_value;
    });
    return {
      labels: Object.keys(grouped),
      data: Object.values(grouped),
    };
  });

  // --- Actions ---

  async function fetchByFiscalYear(year: number) {
    loading.value = true;
    error.value = null;
    selectedFiscalYear.value = year;

    // Logic ปีงบประมาณ: ต.ค. ปีก่อนหน้า - ก.ย. ปีปัจจุบัน
    const startDate = `${year - 1}-10-01`;
    const endDate = `${year}-09-30`;

    try {
      const { data, error: dbError } = await supabase
        .from('med_transactions')
        .select('*')
        .gte('transaction_date', startDate)
        .lte('transaction_date', endDate)
        .order('transaction_date', { ascending: false });

      if (dbError)
        throw dbError;

      transactions.value = data || [];
    }
    catch (err: any) {
      console.error('Fetch Error:', err);
      error.value = err.message || 'Failed to fetch data';
    }
    finally {
      loading.value = false;
    }
  }

  const quarterlySummary = computed(() => {
    const summary = { q1: 0, q2: 0, q3: 0, q4: 0 };

    transactions.value.forEach((t) => {
      const date = new Date(t.transaction_date);
      const month = date.getMonth() + 1; // 1-12
      const value = t.drug_value || 0;

      // Logic ปีงบประมาณไทย (Oct-Sep)
      // Q1: ต.ค. - ธ.ค.
      // Q2: ม.ค. - มี.ค.
      // Q3: เม.ย. - มิ.ย.
      // Q4: ก.ค. - ก.ย.

      if (month >= 10 && month <= 12)
        summary.q1 += value;
      else if (month >= 1 && month <= 3)
        summary.q2 += value;
      else if (month >= 4 && month <= 6)
        summary.q3 += value;
      else if (month >= 7 && month <= 9)
        summary.q4 += value;
    });

    return summary;
  });

  return {
    transactions,
    loading,
    error,
    selectedFiscalYear,
    totalValue,
    totalCount,
    averageValue,
    recentTransactions,
    chartData,
    quarterlySummary,
    fetchByFiscalYear,
  };
});
