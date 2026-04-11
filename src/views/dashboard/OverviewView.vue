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
  <div class="space-y-8 pb-16">
    <!-- Header Section -->
    <div
      class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 bg-cream border border-block-gold p-6"
      style="box-shadow: rgba(127,99,21,0.10) -8px 16px 39px, rgba(127,99,21,0.06) -33px 64px 72px;"
    >
      <div class="flex items-center gap-4">
        <!-- Icon block -->
        <div
          class="w-12 h-12 flex items-center justify-center shrink-0"
          style="background: linear-gradient(135deg, #ffa110, #fa520f);"
        >
          <CalendarRange class="w-6 h-6 text-white" />
        </div>

        <div>
          <h1 class="text-xl text-[#1f1f1f] leading-snug">
            Dashboard รายงานมูลค่ายาสนับสนุน
          </h1>
          <p class="text-sm text-[#1f1f1f]/50 mt-0.5">
            ระบบติดตามงบประมาณสนับสนุนทางการแพทย์
          </p>
        </div>
      </div>

      <!-- Fiscal Year Selector -->
      <div class="flex items-center gap-3 shrink-0">
        <label class="text-xs text-[#1f1f1f]/50 uppercase tracking-wider whitespace-nowrap">
          ปีงบประมาณ
        </label>
        <select
          v-model="store.selectedFiscalYear"
          class="px-4 py-2 bg-warm-ivory border border-block-gold rounded-none text-sm text-[#1f1f1f] outline-none focus:border-[#fa520f] focus:ring-1 focus:ring-[#fa520f] cursor-pointer transition-colors duration-200"
          @change="store.fetchByFiscalYear(store.selectedFiscalYear)"
        >
          <option v-for="y in years" :key="y" :value="y">
            ปี {{ y + 543 }}
          </option>
        </select>
      </div>
    </div>

    <!-- Loading State -->
    <div v-if="store.loading" class="h-80 flex flex-col items-center justify-center text-[#1f1f1f]/40">
      <Loader2 class="w-8 h-8 animate-spin mb-4 text-sunshine-700" />
      <p class="text-sm tracking-wide">
        กำลังประมวลผลข้อมูล...
      </p>
    </div>

    <template v-else>
      <!-- KPI Cards Row -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
        <KpiCard
          title="มูลค่ารวมทั้งปี" :value="formatCurrency(store.totalValue)" :icon="Coins"
          color-class="bg-[#ffd06a] text-[#1f1f1f]"
        />
        <KpiCard
          title="จำนวนรายการทั้งหมด" :value="store.totalCount.toLocaleString()" sub-value="Transactions"
          :icon="Receipt" color-class="bg-[#ffb83e] text-[#1f1f1f]"
        />
        <KpiCard
          title="มูลค่าเฉลี่ยต่อใบยา" :value="formatCurrency(store.averageValue)" :icon="Calculator"
          color-class="bg-[#ffe295] text-[#fa520f]"
        />
      </div>

      <!-- Quarterly Report Section -->
      <div
        class="bg-cream border `border-block-gold"
        style="box-shadow: rgba(127,99,21,0.10) -8px 16px 39px, rgba(127,99,21,0.06) -33px 64px 72px;"
      >
        <!-- Section Header -->
        <div class="px-6 py-5 border-b border-block-gold flex items-center gap-3">
          <span class="w-1 h-5 bg-[#fa520f] shrink-0" />
          <h3 class="text-base text-[#1f1f1f]">
            สรุปรายไตรมาส
            <span class="text-[#1f1f1f]/40 text-sm ml-1">Quarterly Report</span>
          </h3>
        </div>

        <div class="p-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
          <!-- Q1 -->
          <div
            class="bg-warm-ivory border border-block-gold p-5 hover:border-sunshine-700 hover:-translate-y-0.5 transition-all duration-200"
          >
            <p class="text-xs text-[#1f1f1f]/50 mb-2 uppercase tracking-wider">
              ไตรมาส 1
            </p>
            <p class="text-[11px] text-[#1f1f1f]/40 mb-3">
              ต.ค. – ธ.ค.
            </p>
            <div class="text-xl text-[#1f1f1f]">
              {{ formatCurrency(store.quarterlySummary.q1) }}
            </div>
          </div>

          <!-- Q2 -->
          <div
            class="bg-warm-ivory border border-block-gold p-5 hover:border-sunshine-700 hover:-translate-y-0.5 transition-all duration-200"
          >
            <p class="text-xs text-[#1f1f1f]/50 mb-2 uppercase tracking-wider">
              ไตรมาส 2
            </p>
            <p class="text-[11px] text-[#1f1f1f]/40 mb-3">
              ม.ค. – มี.ค.
            </p>
            <div class="text-xl text-[#1f1f1f]">
              {{ formatCurrency(store.quarterlySummary.q2) }}
            </div>
          </div>

          <!-- Q3 -->
          <div
            class="bg-warm-ivory border border-block-gold p-5 hover:border-sunshine-700 hover:-translate-y-0.5 transition-all duration-200"
          >
            <p class="text-xs text-[#1f1f1f]/50 mb-2 uppercase tracking-wider">
              ไตรมาส 3
            </p>
            <p class="text-[11px] text-[#1f1f1f]/40 mb-3">
              เม.ย. – มิ.ย.
            </p>
            <div class="text-xl text-[#1f1f1f]">
              {{ formatCurrency(store.quarterlySummary.q3) }}
            </div>
          </div>

          <!-- Q4 -->
          <div
            class="bg-warm-ivory border border-block-gold p-5 hover:border-sunshine-700 hover:-translate-y-0.5 transition-all duration-200"
          >
            <p class="text-xs text-[#1f1f1f]/50 mb-2 uppercase tracking-wider">
              ไตรมาส 4
            </p>
            <p class="text-[11px] text-[#1f1f1f]/40 mb-3">
              ก.ค. – ก.ย.
            </p>
            <div class="text-xl text-[#1f1f1f]">
              {{ formatCurrency(store.quarterlySummary.q4) }}
            </div>
          </div>
        </div>
      </div>

      <!-- Recent Transactions Table -->
      <div
        class="bg-cream border border-block-gold overflow-hidden"
        style="box-shadow: rgba(127,99,21,0.10) -8px 16px 39px, rgba(127,99,21,0.06) -33px 64px 72px;"
      >
        <!-- Section Header -->
        <div class="px-6 py-5 border-b border-block-gold flex items-center gap-3">
          <span class="w-1 h-5 bg-[#fa520f] shrink-0" />
          <h3 class="text-base text-[#1f1f1f]">
            รายการล่าสุด
          </h3>
        </div>

        <div class="overflow-x-auto">
          <table class="w-full text-sm text-left">
            <thead class="bg-block-gold text-[#1f1f1f]/60">
              <tr>
                <th class="px-6 py-4 text-xs uppercase tracking-wider font-normal">
                  วันที่
                </th>
                <th class="px-6 py-4 text-xs uppercase tracking-wider font-normal">
                  เลขที่บิล
                </th>
                <th class="px-6 py-4 text-xs uppercase tracking-wider font-normal">
                  ประเภทยา
                </th>
                <th class="px-6 py-4 text-xs uppercase tracking-wider font-normal text-right">
                  มูลค่า (บาท)
                </th>
              </tr>
            </thead>
            <tbody class="divide-y divide-block-gold">
              <tr
                v-for="item in store.recentTransactions" :key="item.id"
                class="hover:bg-warm-ivory transition-colors duration-150"
              >
                <td class="px-6 py-4 text-[#1f1f1f]/60">
                  {{ new Date(item.transaction_date).toLocaleDateString('th-TH') }}
                </td>
                <td class="px-6 py-4 font-mono text-[#1f1f1f]">
                  {{ item.bill_number || '-' }}
                </td>
                <td class="px-6 py-4">
                  <span class="px-3 py-1 bg-warm-ivory border border-block-gold text-xs text-[#fa520f]">
                    {{ item.drug_type }}
                  </span>
                </td>
                <td class="px-6 py-4 text-right text-[#1f1f1f]">
                  {{ formatCurrency(item.drug_value) }}
                </td>
              </tr>

              <!-- Empty State -->
              <tr v-if="store.recentTransactions.length === 0">
                <td colspan="4" class="px-6 py-16 text-center bg-warm-ivory">
                  <div class="flex flex-col items-center gap-3 text-[#1f1f1f]/30">
                    <Receipt class="w-8 h-8" />
                    <p class="text-sm">
                      ไม่พบข้อมูลรายการในปีงบประมาณนี้
                    </p>
                  </div>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
    </template>
  </div>
</template>
