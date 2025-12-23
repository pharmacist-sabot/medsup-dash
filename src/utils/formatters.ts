// src/utils/formatters.ts
export function formatCurrency(amount: number): string {
  return new Intl.NumberFormat('th-TH', {
    style: 'currency',
    currency: 'THB',
    minimumFractionDigits: 2,
  }).format(amount);
}

export function formatDate(dateString: string): string {
  if (!dateString)
    return '-';
  return new Intl.DateTimeFormat('th-TH', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  }).format(new Date(dateString));
}
