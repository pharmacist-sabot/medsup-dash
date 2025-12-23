<!-- src/views/dashboard/OverviewView.vue -->
<script setup lang="ts">
import { Calculator, CalendarRange, Coins, Loader2, Receipt } from 'lucide-vue-next';
import { onMounted } from 'vue';

import KpiCard from '@/components/dashboard/KpiCard.vue';
import { useTransactionStore } from '@/stores/transactions';
import { formatCurrency } from '@/utils/formatters';

const store = useTransactionStore();

onMounted(() => {
  store.fetchByFiscalYear(store.selectedFiscalYear);
});

// Calculate Current Fiscal Year (Oct of prev year starts new fiscal year)
const currentFiscalYear = new Date().getFullYear() + (new Date().getMonth() >= 9 ? 1 : 0);
const startYear = 2023;
// Generate years from current fiscal year down to startYear
const years = Array.from(
  { length: currentFiscalYear - startYear + 1 },
  (_, i) => currentFiscalYear - i,
);
</script>

<template>
  <div class="space-y-8 pb-12">
    <!-- Header Section -->
    <div class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 bg-white/60 backdrop-blur-sm p-6 rounded-2xl border border-white/50 shadow-sm">
      <div class="flex items-center gap-4">
        <div class="p-3 bg-brand/10 rounded-xl text-brand-dark">
          <CalendarRange class="w-8 h-8" />
        </div>
        <div>
          <h1 class="text-2xl font-bold text-gray-800">
            Dashboard รายงานมูลค่ายาสนับสนุน
          </h1>
          <p class="text-gray-600 text-sm">
            ระบบติดตามงบประมาณสนับสนุนทางการแพทย์
          </p>
        </div>
      </div>

      <div class="flex items-center gap-3">
        <label class="text-sm font-medium text-gray-700">ปีงบประมาณ:</label>
        <select
          v-model="store.selectedFiscalYear"
          class="px-4 py-2 bg-white border border-purple-200 rounded-xl text-sm font-medium text-brand-dark focus:ring-2 focus:ring-brand focus:border-brand outline-none shadow-sm cursor-pointer hover:border-brand transition-colors"
          @change="store.fetchByFiscalYear(store.selectedFiscalYear)"
        >
          <option v-for="y in years" :key="y" :value="y">
            ปี {{ y + 543 }}
          </option>
        </select>
      </div>
    </div>

    <!-- Loading -->
    <div v-if="store.loading" class="h-96 flex flex-col items-center justify-center text-brand-dark/60">
      <Loader2 class="w-10 h-10 animate-spin mb-4" />
      <p class="font-medium">
        กำลังประมวลผลข้อมูล...
      </p>
    </div>

    <template v-else>
      <!-- KPI Cards Row -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <KpiCard
          title="มูลค่ารวมทั้งปี"
          :value="formatCurrency(store.totalValue)"
          :icon="Coins"
          color-class="bg-fuchsia-100 text-fuchsia-600"
        />
        <KpiCard
          title="จำนวนรายการทั้งหมด"
          :value="store.totalCount.toLocaleString()"
          sub-value="Transactions"
          :icon="Receipt"
          color-class="bg-violet-100 text-violet-600"
        />
        <KpiCard
          title="มูลค่าเฉลี่ยต่อใบยา"
          :value="formatCurrency(store.averageValue)"
          :icon="Calculator"
          color-class="bg-pink-100 text-pink-600"
        />
      </div>

      <!-- Quarterly Report Section -->
      <div class="bg-white/80 backdrop-blur rounded-2xl border border-purple-100 shadow-sm p-6">
        <h3 class="text-lg font-bold text-gray-800 mb-6 flex items-center gap-2">
          <span class="w-1.5 h-6 bg-brand rounded-full" />
          สรุปรายไตรมาส (Quarterly Report)
        </h3>

        <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <!-- Q1 -->
          <div class="bg-purple-50/50 p-5 rounded-xl border border-purple-100 hover:border-brand/30 transition-all hover:-translate-y-1">
            <p class="text-sm text-gray-500 mb-1">
              ไตรมาส 1 (ต.ค.-ธ.ค.)
            </p>
            <h4 class="text-xl font-bold text-brand-dark">
              {{ formatCurrency(store.quarterlySummary.q1) }}
            </h4>
          </div>
          <!-- Q2 -->
          <div class="bg-purple-50/50 p-5 rounded-xl border border-purple-100 hover:border-brand/30 transition-all hover:-translate-y-1">
            <p class="text-sm text-gray-500 mb-1">
              ไตรมาส 2 (ม.ค.-มี.ค.)
            </p>
            <h4 class="text-xl font-bold text-brand-dark">
              {{ formatCurrency(store.quarterlySummary.q2) }}
            </h4>
          </div>
          <!-- Q3 -->
          <div class="bg-purple-50/50 p-5 rounded-xl border border-purple-100 hover:border-brand/30 transition-all hover:-translate-y-1">
            <p class="text-sm text-gray-500 mb-1">
              ไตรมาส 3 (เม.ย.-มิ.ย.)
            </p>
            <h4 class="text-xl font-bold text-brand-dark">
              {{ formatCurrency(store.quarterlySummary.q3) }}
            </h4>
          </div>
          <!-- Q4 -->
          <div class="bg-purple-50/50 p-5 rounded-xl border border-purple-100 hover:border-brand/30 transition-all hover:-translate-y-1">
            <p class="text-sm text-gray-500 mb-1">
              ไตรมาส 4 (ก.ค.-ก.ย.)
            </p>
            <h4 class="text-xl font-bold text-brand-dark">
              {{ formatCurrency(store.quarterlySummary.q4) }}
            </h4>
          </div>
        </div>
      </div>

      <!-- Table Section -->
      <div class="bg-white/80 backdrop-blur rounded-2xl border border-purple-100 shadow-sm overflow-hidden">
        <div class="px-6 py-5 border-b border-purple-50 flex justify-between items-center">
          <h3 class="text-lg font-bold text-gray-800 flex items-center gap-2">
            <span class="w-1.5 h-6 bg-brand rounded-full" />
            รายการล่าสุด
          </h3>
        </div>
        <div class="overflow-x-auto">
          <table class="w-full text-sm text-left">
            <thead class="bg-purple-50/50 text-gray-600 font-medium">
              <tr>
                <th class="px-6 py-4">
                  วันที่
                </th>
                <th class="px-6 py-4">
                  เลขที่บิล
                </th>
                <th class="px-6 py-4">
                  ประเภทยา
                </th>
                <th class="px-6 py-4 text-right">
                  มูลค่า (บาท)
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-purple-50">
              <tr
                v-for="item in store.recentTransactions"
                :key="item.id"
                class="hover:bg-purple-50/30 transition-colors"
              >
                <td class="px-6 py-4 text-gray-600">
                  {{ new Date(item.transaction_date).toLocaleDateString('th-TH') }}
                </td>
                <td class="px-6 py-4 font-mono text-brand-dark font-medium">
                  {{ item.bill_number || '-' }}
                </td>
                <td class="px-6 py-4">
                  <span class="px-3 py-1 rounded-full bg-white border border-purple-100 text-xs font-medium text-purple-600 shadow-sm">
                    {{ item.drug_type }}
                  </span>
                </td>
                <td class="px-6 py-4 text-right font-bold text-gray-700">
                  {{ formatCurrency(item.drug_value) }}
                </td>
              </tr>
              <tr v-if="store.recentTransactions.length === 0">
                <td colspan="4" class="px-6 py-12 text-center text-gray-400 bg-white">
                  ไม่พบข้อมูลรายการในปีงบประมาณนี้
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>
