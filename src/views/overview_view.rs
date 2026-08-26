use leptos::prelude::*;

use crate::components::dashboard::kpi_card::KpiCard;
use crate::components::icons::{Icon, IconKind};
use crate::core::types::database::MedTransaction;
use crate::core::utils::{format_currency, format_date, select_value, selectable_fiscal_years};
use crate::stores::transactions::use_transactions;

#[component]
pub fn OverviewView() -> impl IntoView {
    let store = use_transactions();

    // onMounted equivalent: initial fetch + periodic refresh subscription.
    store.subscribe_refresh();
    let initial_year = store.selected_fiscal_year.get_untracked();
    leptos::task::spawn_local(async move {
        use_transactions().fetch_by_fiscal_year(initial_year).await;
    });
    on_cleanup(move || store.unsubscribe_refresh());

    let total_value = store.total_value();
    let total_count = store.total_count();
    let average_value = store.average_value();
    let quarterly = store.quarterly_summary();
    let recent = store.recent_transactions();
    let loading = store.loading;
    let error = store.error;
    let selected_year = store.selected_fiscal_year;

    let years = selectable_fiscal_years();

    let handle_year_change = move |ev: web_sys::Event| {
        if let Ok(year) = select_value(&ev).parse::<i32>() {
            let store = use_transactions();
            leptos::task::spawn_local(async move {
                store.fetch_by_fiscal_year(year).await;
            });
        }
    };

    view! {
        <div class="space-y-8 pb-16">
            // Header Section
            <div
                class="flex flex-col sm:flex-row justify-between items-start sm:items-center gap-4 bg-cream border border-block-gold p-6"
                style="box-shadow: rgba(127,99,21,0.10) -8px 16px 39px, rgba(127,99,21,0.06) -33px 64px 72px;"
            >
                <div class="flex items-center gap-4">
                    // Icon block
                    <div
                        class="w-12 h-12 flex items-center justify-center shrink-0"
                        style="background: linear-gradient(135deg, #ffa110, #fa520f);"
                    >
                        <Icon icon=IconKind::CalendarRange class="w-6 h-6 text-white" aria_hidden=true />
                    </div>

                    <div>
                        <h1 class="text-xl text-mistral-black leading-snug">
                            "Dashboard รายงานมูลค่ายาสนับสนุน"
                        </h1>
                        <p class="text-sm text-mistral-black/50 mt-0.5">
                            "ระบบติดตามงบประมาณสนับสนุนทางการแพทย์"
                        </p>
                    </div>
                </div>

                // Fiscal Year Selector
                <div class="flex items-center gap-3 shrink-0">
                    <label for="fiscal-year" class="text-xs text-mistral-black/50 uppercase tracking-wider whitespace-nowrap">
                        "ปีงบประมาณ"
                    </label>
                    <select
                        id="fiscal-year"
                        class="px-4 py-2 bg-warm-ivory border border-block-gold rounded-none text-sm text-mistral-black outline-none focus:border-mistral-orange focus:ring-1 focus:ring-mistral-orange cursor-pointer transition-colors duration-200"
                        on:change=handle_year_change
                    >
                        {years
                            .into_iter()
                            .map(|y| {
                                view! {
                                    <option
                                        value=y.to_string()
                                        selected=Signal::derive(move || y == selected_year.get())
                                    >
                                        "ปี " {y + 543}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </select>
                </div>
            </div>

            // Loading State (v-if) / Dashboard Body (v-else)
            <Show
                when=move || loading.get()
                fallback=move || {
                    view! {
                        <>
                            <Show when=move || error.get().is_some()>
                                <div
                                    role="alert"
                                    class="bg-cream border border-mistral-orange/30 p-4 text-sm text-mistral-orange"
                                >
                                    {move || error.get().unwrap_or_default()}
                                </div>
                            </Show>

                            // KPI Cards Row
                            <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                                <KpiCard
                                    title="มูลค่ารวมทั้งปี"
                                    value=Signal::derive(move || format_currency(total_value.get()))
                                    icon=IconKind::Coins
                                    color_class="bg-sunshine-300 text-mistral-black"
                                />
                                <KpiCard
                                    title="จำนวนรายการทั้งหมด"
                                    value=Signal::derive(move || total_count.get().to_string())
                                    sub_value="Transactions".to_string()
                                    icon=IconKind::Receipt
                                    color_class="bg-sunshine-500 text-mistral-black"
                                />
                                <KpiCard
                                    title="มูลค่าเฉลี่ยต่อใบยา"
                                    value=Signal::derive(move || format_currency(average_value.get()))
                                    icon=IconKind::Calculator
                                    color_class="bg-block-gold text-mistral-orange"
                                />
                            </div>

                            <QuarterlyReport quarterly=quarterly />
                            <RecentTable recent=recent />
                        </>
                    }
                }
            >
                <div class="h-80 flex flex-col items-center justify-center text-mistral-black/50">
                    <Icon icon=IconKind::Loader2 class="w-8 h-8 animate-spin mb-4 text-sunshine-700" aria_hidden=true />
                    <p class="text-sm tracking-wide">"กำลังประมวลผลข้อมูล..."</p>
                </div>
            </Show>
        </div>
    }
}

#[component]
fn QuarterlyReport(
    quarterly: leptos::prelude::Memo<crate::stores::transactions::QuarterlySummary>,
) -> impl IntoView {
    view! {
        <div
            class="bg-cream"
            style="box-shadow: rgba(127,99,21,0.10) -8px 16px 39px, rgba(127,99,21,0.06) -33px 64px 72px;"
        >
            // Section Header
            <div class="px-6 py-5 flex items-center gap-3">
                <span class="w-1 h-5 bg-mistral-orange shrink-0"></span>
                <h3 class="text-base text-mistral-black">
                    "สรุปรายไตรมาส "
                    <span class="text-mistral-black/50 text-sm ml-1">Quarterly Report</span>
                </h3>
            </div>

            <div class="p-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
                <QuarterCard
                    label="ไตรมาส 1"
                    range="ต.ค. - ธ.ค."
                    value=Signal::derive(move || format_currency(quarterly.get().q1))
                />
                <QuarterCard
                    label="ไตรมาส 2"
                    range="ม.ค. - มี.ค."
                    value=Signal::derive(move || format_currency(quarterly.get().q2))
                />
                <QuarterCard
                    label="ไตรมาส 3"
                    range="เม.ย. - มิ.ย."
                    value=Signal::derive(move || format_currency(quarterly.get().q3))
                />
                <QuarterCard
                    label="ไตรมาส 4"
                    range="ก.ค. - ก.ย."
                    value=Signal::derive(move || format_currency(quarterly.get().q4))
                />
            </div>
        </div>
    }
}

#[component]
fn QuarterCard(
    #[prop(into)] label: String,
    #[prop(into)] range: String,
    #[prop(into)] value: Signal<String>,
) -> impl IntoView {
    view! {
        <div class="bg-warm-ivory p-5 hover:-translate-y-0.5 transition-all duration-200">
            <p class="text-xs text-mistral-black/50 mb-2 uppercase tracking-wider">{label}</p>
            <p class="text-[11px] text-mistral-black/50 mb-3">{range}</p>
            <div class="text-xl text-mistral-black">{move || value.get()}</div>
        </div>
    }
}

#[component]
fn RecentTable(recent: leptos::prelude::Memo<Vec<MedTransaction>>) -> impl IntoView {
    view! {
        <div
            class="bg-cream border border-block-gold overflow-hidden"
            style="box-shadow: rgba(127,99,21,0.10) -8px 16px 39px, rgba(127,99,21,0.06) -33px 64px 72px;"
        >
            // Section Header
            <div class="px-6 py-5 border-b border-block-gold flex items-center gap-3">
                <span class="w-1 h-5 bg-mistral-orange shrink-0"></span>
                <h3 class="text-base text-mistral-black">"รายการล่าสุด"</h3>
            </div>

            <div class="overflow-x-auto">
                <table class="w-full text-sm text-left">
                    <thead class="bg-block-gold text-mistral-black/60">
                        <tr>
                            <th scope="col" class="px-6 py-4 text-xs uppercase tracking-wider font-normal">
                                "วันที่"
                            </th>
                            <th scope="col" class="px-6 py-4 text-xs uppercase tracking-wider font-normal">
                                "เลขที่บิล"
                            </th>
                            <th scope="col" class="px-6 py-4 text-xs uppercase tracking-wider font-normal">
                                "ประเภทยา"
                            </th>
                            <th scope="col" class="px-6 py-4 text-xs uppercase tracking-wider font-normal text-right">
                                "มูลค่า (บาท)"
                            </th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-block-gold">
                        <For
                            each=move || recent.get()
                            key=|item| item.id.clone()
                            children=move |item| {
                                view! {
                                    <tr class="hover:bg-warm-ivory transition-colors duration-150">
                                        <td class="px-6 py-4 text-mistral-black/60">
                                            {format_date(Some(&item.transaction_date))}
                                        </td>
                                        <td class="px-6 py-4 font-mono text-mistral-black">
                                            {item.bill_number.clone().unwrap_or_else(|| "-".to_string())}
                                        </td>
                                        <td class="px-6 py-4">
                                            <span class="px-3 py-1 bg-warm-ivory border border-block-gold text-xs text-mistral-orange">
                                                {item.drug_type.clone()}
                                            </span>
                                        </td>
                                        <td class="px-6 py-4 text-right text-mistral-black">
                                            {format_currency(item.drug_value)}
                                        </td>
                                    </tr>
                                }
                            }
                        />

                        // Empty State
                        <Show when=move || recent.with(Vec::is_empty)>
                            <tr>
                                <td colspan="4" class="px-6 py-16 text-center bg-warm-ivory">
                                    <div class="flex flex-col items-center gap-3 text-mistral-black/50">
                                        <Icon icon=IconKind::Receipt class="w-8 h-8" aria_hidden=true />
                                        <p class="text-sm">"ไม่พบข้อมูลรายการในปีงบประมาณนี้"</p>
                                    </div>
                                </td>
                            </tr>
                        </Show>
                    </tbody>
                </table>
            </div>
        </div>
    }
}
