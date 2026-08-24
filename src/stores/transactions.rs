use std::cell::RefCell;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU32, Ordering};

use leptos::prelude::*;
use serde::Serialize;
use wasm_bindgen::{JsCast, closure::Closure};

use crate::core::error::AppResult;
use crate::core::supabase::supabase;
use crate::core::time;
use crate::core::types::database::MedTransaction;

static TRANSACTIONS: OnceLock<TransactionsState> = OnceLock::new();
static REQUEST_SEQ: AtomicU32 = AtomicU32::new(0);

/// Supabase Realtime (websocket) is not part of the PostgREST/GoTrue surface
/// this app implements, so change notifications are replaced by periodic
/// polling with the same refetch behaviour.
const REFRESH_POLL_INTERVAL_MS: i32 = 30_000;

struct PollHandle {
    id: i32,
    closure: Closure<dyn FnMut()>,
}

thread_local! {
    /// Active poll handle. WASM is single-threaded, so a thread-local is both
    /// sufficient and `Send`-constraint-free for the non-`Send` JS closure.
    static POLL: RefCell<Option<PollHandle>> = const { RefCell::new(None) };
}

/// Create the singleton. Called once inside the mount root owner.
pub fn install() {
    let _ = TRANSACTIONS.set(TransactionsState::new());
}

/// Access the installed singleton.
///
/// # Panics
/// Panics if [`install`] has not run yet (i.e. before mount).
#[must_use]
pub fn use_transactions() -> TransactionsState {
    *TRANSACTIONS
        .get()
        .expect("TransactionsState not initialized")
}

/// Quarterly totals for a Thai fiscal year (Q1 = Oct-Dec).
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
pub struct QuarterlySummary {
    pub q1: f64,
    pub q2: f64,
    pub q3: f64,
    pub q4: f64,
}

fn quarterly_summary_of(transactions: &[MedTransaction]) -> QuarterlySummary {
    let mut summary = QuarterlySummary::default();
    for t in transactions {
        // transaction_date is "YYYY-MM-DD"; slice the month directly to stay
        // allocation-light and timezone-independent.
        let month: u32 = t
            .transaction_date
            .get(5..7)
            .and_then(|m| m.parse().ok())
            .unwrap_or(0);
        let value = t.drug_value;
        match month {
            10..=12 => summary.q1 += value, // ต.ค. – ธ.ค.
            1..=3 => summary.q2 += value,   // ม.ค. – มี.ค.
            4..=6 => summary.q3 += value,   // เม.ย. – มิ.ย.
            7..=9 => summary.q4 += value,   // ก.ค. – ก.ย.
            _ => {}
        }
    }
    summary
}

/// Store state mirroring the old Pinia `transactions` store, with the
/// computed metrics expressed as Leptos memos.
#[derive(Debug, Clone, Copy)]
pub struct TransactionsState {
    pub transactions: RwSignal<Vec<MedTransaction>>,
    pub loading: RwSignal<bool>,
    pub error: RwSignal<Option<String>>,
    pub selected_fiscal_year: RwSignal<i32>,
    total_value: Memo<f64>,
    total_count: Memo<usize>,
    average_value: Memo<f64>,
    recent_transactions: Memo<Vec<MedTransaction>>,
    quarterly_summary: Memo<QuarterlySummary>,
}

impl TransactionsState {
    fn new() -> Self {
        let transactions = RwSignal::new(Vec::new());
        let loading = RwSignal::new(false);
        let error = RwSignal::new(None);
        let selected_fiscal_year = RwSignal::new(time::current_fiscal_year());

        let total_value = Memo::new(move |_| {
            transactions
                .with(|rows: &Vec<MedTransaction>| rows.iter().map(|t| t.drug_value).sum::<f64>())
        });
        let total_count = Memo::new(move |_| transactions.with(Vec::len));
        let average_value = Memo::new(move |_| {
            #[allow(clippy::cast_precision_loss)]
            // row counts are small; precision loss is irrelevant
            let count = total_count.get() as f64;
            if count > 0.0 {
                total_value.get() / count
            } else {
                0.0
            }
        });
        let recent_transactions = Memo::new(move |_| {
            let mut rows = transactions.get_untracked();
            rows.sort_by(|a, b| b.transaction_date.cmp(&a.transaction_date));
            rows.truncate(10);
            rows
        });
        let quarterly_summary =
            Memo::new(move |_| quarterly_summary_of(&transactions.get_untracked()));

        Self {
            transactions,
            loading,
            error,
            selected_fiscal_year,
            total_value,
            total_count,
            average_value,
            recent_transactions,
            quarterly_summary,
        }
    }

    #[must_use]
    pub const fn total_value(&self) -> Memo<f64> {
        self.total_value
    }

    #[must_use]
    pub const fn total_count(&self) -> Memo<usize> {
        self.total_count
    }

    #[must_use]
    pub const fn average_value(&self) -> Memo<f64> {
        self.average_value
    }

    #[must_use]
    pub const fn recent_transactions(&self) -> Memo<Vec<MedTransaction>> {
        self.recent_transactions
    }

    #[must_use]
    pub const fn quarterly_summary(&self) -> Memo<QuarterlySummary> {
        self.quarterly_summary
    }

    /// Fetch all transactions inside the fiscal year range.
    ///
    /// Stale responses from superseded requests are discarded via a sequence
    /// counter (mirrors `requestSeq` in the original store).
    pub async fn fetch_by_fiscal_year(&self, year: i32) {
        let seq = REQUEST_SEQ.fetch_add(1, Ordering::Relaxed).wrapping_add(1);
        self.loading.set(true);
        self.error.set(None);
        self.selected_fiscal_year.set(year);

        let (start_date, end_date) = time::fiscal_year_range(year);
        let result: AppResult<Vec<MedTransaction>> = async {
            let client = supabase()?;
            client
                .postgrest()
                .from("med_transactions")
                .select("*")
                .gte("transaction_date", &start_date)
                .lte("transaction_date", &end_date)
                .order("transaction_date", false)
                .get()
                .await
        }
        .await;

        if REQUEST_SEQ.load(Ordering::Relaxed) != seq {
            return; // a newer request superseded this one
        }

        match result {
            Ok(rows) => {
                self.transactions.set(rows);
            }
            Err(err) => {
                log::error!("Fetch Error: {err}");
                self.error.set(Some(err.to_string()));
                self.transactions.set(Vec::new());
            }
        }

        self.loading.set(false);
    }

    /// Start periodic refreshing (replaces the Supabase Realtime channel).
    /// No-op while already subscribed or while a fetch is in flight.
    pub fn subscribe_refresh(&self) {
        POLL.with_borrow_mut(|slot| {
            if slot.is_some() {
                return;
            }

            let state = *self;
            let closure = Closure::wrap(Box::new(move || {
                let year = state.selected_fiscal_year.get_untracked();
                if !state.loading.get_untracked() {
                    leptos::task::spawn_local(async move {
                        state.fetch_by_fiscal_year(year).await;
                    });
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                match window.set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    REFRESH_POLL_INTERVAL_MS,
                ) {
                    Ok(id) => *slot = Some(PollHandle { id, closure }),
                    Err(err) => log::error!("Failed to start refresh polling: {err:?}"),
                }
            }
        });
    }

    /// Stop periodic refreshing and release the interval handle.
    pub fn unsubscribe_refresh(&self) {
        POLL.with_borrow_mut(|slot| {
            if let Some(handle) = slot.take() {
                if let Some(window) = web_sys::window() {
                    window.clear_interval_with_handle(handle.id);
                }
                drop(handle.closure);
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{QuarterlySummary, quarterly_summary_of};
    use crate::core::types::database::MedTransaction;

    fn row(date: &str, value: f64) -> MedTransaction {
        MedTransaction {
            id: 1,
            created_at: None,
            transaction_date: date.to_string(),
            bill_number: None,
            drug_type: "Generic".to_string(),
            drug_value: value,
            note: None,
        }
    }

    #[test]
    fn buckets_values_into_thai_fiscal_quarters() {
        let rows = [
            row("2024-10-05", 100.0), // Q1
            row("2024-12-31", 50.0),  // Q1
            row("2025-01-15", 200.0), // Q2
            row("2025-04-02", 25.5),  // Q3
            row("2025-09-30", 24.5),  // Q4
            row("2025-03-01", 100.0), // Q2
        ];
        let summary = quarterly_summary_of(&rows);
        assert_eq!(
            summary,
            QuarterlySummary {
                q1: 150.0,
                q2: 300.0,
                q3: 25.5,
                q4: 24.5
            }
        );
    }

    #[test]
    fn empty_input_gives_zeroed_summary() {
        assert_eq!(quarterly_summary_of(&[]), QuarterlySummary::default());
    }
}
