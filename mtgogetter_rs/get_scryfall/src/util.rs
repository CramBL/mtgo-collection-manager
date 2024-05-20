use chrono::{DateTime, Utc};

/// Parse a [`DateTime<Utc>`] to a string of the form `YYYYMMDDHHMMSS` e.g. `20240519090548`
///
/// # Example
/// ```
/// # use get_scryfall::util::format_datetime_utc_for_url;
/// use chrono::{Utc, TimeZone};
/// let date = Utc.with_ymd_and_hms(2024, 5, 19, 9, 5, 48).unwrap();
/// let formatted = format_datetime_utc_for_url(date);
/// assert_eq!(formatted, "20240519090548");
/// ```
pub fn format_datetime_utc_for_url(date: DateTime<Utc>) -> String {
    date.format("%Y%m%d%H%M%S").to_string()
}

#[allow(dead_code)]
#[cfg(debug_assertions)]
pub fn init_debug_logging<V>(verbosity: V)
where
    V: Into<stderrlog::LogLevelNum>,
{
    stderrlog::new().verbosity(verbosity).init().unwrap();
}
