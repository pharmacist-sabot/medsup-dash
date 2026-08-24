use chrono::{Datelike, Utc};

/// First fiscal year available in the dashboard.
pub const START_YEAR: i32 = 2023;

/// Thai fiscal year boundary: a new fiscal year starts in October, so from
/// October through December the fiscal year is `calendar year + 1`.
#[must_use]
pub fn current_fiscal_year() -> i32 {
    let now = Utc::now();
    if now.month() >= 10 {
        now.year() + 1
    } else {
        now.year()
    }
}

/// Build the `PostgREST` date bounds for a fiscal year: `(start, end)` as
/// inclusive `YYYY-MM-DD` strings.
#[must_use]
pub fn fiscal_year_range(year: i32) -> (String, String) {
    (format!("{}-10-01", year - 1), format!("{year}-09-30"))
}

/// Fiscal years selectable in the UI, newest first.
#[must_use]
pub fn fiscal_years_desc() -> Vec<i32> {
    let current = current_fiscal_year();
    (START_YEAR..=current).rev().collect()
}

#[cfg(test)]
mod tests {
    use super::{START_YEAR, current_fiscal_year, fiscal_year_range, fiscal_years_desc};

    #[test]
    fn range_spends_october_to_september() {
        let (start, end) = fiscal_year_range(2567);
        assert_eq!(start, "2566-10-01");
        assert_eq!(end, "2567-09-30");
    }

    #[test]
    fn years_are_descending_from_current() {
        let years = fiscal_years_desc();
        assert_eq!(years.first(), Some(&current_fiscal_year()));
        assert_eq!(*years.last().expect("non-empty"), START_YEAR);
        assert!(
            years.windows(2).all(|w| w[0] > w[1]),
            "years must be sorted descending"
        );
    }
}
