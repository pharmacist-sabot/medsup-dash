use wasm_bindgen::JsCast;

use crate::core::time;

const THAI_SHORT_MONTHS: [&str; 12] = [
    "ม.ค.",
    "ก.พ.",
    "มี.ค.",
    "เม.ย.",
    "พ.ค.",
    "มิ.ย.",
    "ก.ค.",
    "ส.ค.",
    "ก.ย.",
    "ต.ค.",
    "พ.ย.",
    "ธ.ค.",
];

/// Format an amount as Thai Baht currency: `format_currency(1234.56)` →
/// `"฿1,234.56"` (matches `Intl.NumberFormat("th-TH", { style: "currency",
/// currency: "THB" })` output shape).
#[must_use]
pub fn format_currency(amount: f64) -> String {
    let rounded = format!("{amount:.2}");
    let (integer, fraction) = match rounded.split_once('.') {
        Some((i, f)) => (i, f),
        None => (rounded.as_str(), "00"),
    };
    let negative = integer.starts_with('-');
    let digits = if negative { &integer[1..] } else { integer };

    let mut grouped = String::with_capacity(digits.len() + digits.len() / 3);
    for (idx, c) in digits.chars().enumerate() {
        if idx > 0 && (digits.len() - idx) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(c);
    }

    if negative {
        format!("-฿{grouped}.{fraction}")
    } else {
        format!("฿{grouped}.{fraction}")
    }
}

/// Format a `YYYY-MM-DD` date string as a short Thai date, e.g.
/// `"2023-12-25"` → `"25 ธ.ค. 2566"` (Buddhist Era year, matching the
/// original `Intl.DateTimeFormat("th-TH", ...)` output).
///
/// Returns `"-"` for missing or malformed input, mirroring the old helper.
#[must_use]
pub fn format_date(date_string: Option<&str>) -> String {
    let Some(raw) = date_string else {
        return "-".to_string();
    };
    // Accept full ISO timestamps by taking the date component only.
    let date_part = raw.split('T').next().unwrap_or_default();
    let parts: Vec<&str> = date_part.split('-').collect();
    if parts.len() != 3 {
        return "-".to_string();
    }
    let Ok(year) = parts[0].parse::<i32>() else {
        return "-".to_string();
    };
    let Ok(month) = parts[1].parse::<u32>() else {
        return "-".to_string();
    };
    let Ok(day) = parts[2].parse::<u32>() else {
        return "-".to_string();
    };
    if !(1..=12).contains(&month) || day == 0 || day > 31 {
        return "-".to_string();
    }
    format!(
        "{} {} {}",
        day,
        THAI_SHORT_MONTHS[(month - 1) as usize],
        year + 543
    )
}

/// Fiscal years shown in the selector (delegates to [`time`]).
#[must_use]
pub fn selectable_fiscal_years() -> Vec<i32> {
    time::fiscal_years_desc()
}

/// Read the current value of the element that fired an input/change event
/// (`<input>` variant).
#[must_use]
pub fn input_value(event: &web_sys::Event) -> String {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
        .map(|element| element.value())
        .unwrap_or_default()
}

/// Read the current value of the element that fired a change event
/// (`<select>` variant).
#[must_use]
pub fn select_value(event: &web_sys::Event) -> String {
    event
        .target()
        .and_then(|target| target.dyn_into::<web_sys::HtmlSelectElement>().ok())
        .map(|element| element.value())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::{format_currency, format_date};

    #[test]
    fn formats_thb_correctly() {
        assert_eq!(format_currency(1234.56), "฿1,234.56");
    }

    #[test]
    fn formats_large_amounts_with_grouping() {
        assert_eq!(format_currency(1_234_567.89), "฿1,234,567.89");
        assert_eq!(format_currency(0.0), "฿0.00");
    }

    #[test]
    fn formats_date_correctly() {
        assert_eq!(format_date(Some("2023-12-25")), "25 ธ.ค. 2566");
        assert_eq!(format_date(Some("2023-12-25T10:30:00Z")), "25 ธ.ค. 2566");
    }

    #[test]
    fn handles_invalid_dates_gracefully() {
        assert_eq!(format_date(Some("")), "-");
        assert_eq!(format_date(None), "-");
    }

    #[test]
    fn returns_dash_for_malformed_date_strings() {
        assert_eq!(format_date(Some("not-a-date")), "-");
        assert_eq!(format_date(Some("2023-13-99")), "-");
        assert_eq!(format_date(Some("2023-00-01")), "-");
    }
}
