use serde::{Deserialize, Serialize};

/// Row shape of the `med_transactions` table (`bigint id`, date strings in
/// `YYYY-MM-DD`). Field names stay `snake_case` to match `PostgREST` columns.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MedTransaction {
    pub id: i64,
    #[serde(default)]
    pub created_at: Option<String>,
    pub transaction_date: String,
    #[serde(default)]
    pub bill_number: Option<String>,
    pub drug_type: String,
    #[serde(default)]
    pub drug_value: f64,
    #[serde(default)]
    pub note: Option<String>,
}
