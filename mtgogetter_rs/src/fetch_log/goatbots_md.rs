#![allow(dead_code)]
use chrono::{DateTime, NaiveDateTime, NaiveTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct GoatBotsMetaData {
    card_definitions_updated_at: Option<DateTime<Utc>>,
    prices_updated_at: Option<DateTime<Utc>>,
}

impl GoatBotsMetaData {
    pub fn new(
        card_definitions_updated_at: Option<DateTime<Utc>>,
        prices_updated_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            card_definitions_updated_at,
            prices_updated_at,
        }
    }

    pub fn card_definitions_updated_at(&self) -> Option<DateTime<Utc>> {
        self.card_definitions_updated_at
    }

    pub(super) fn refresh_card_definitions_updated_at_timestamp(&mut self) {
        self.card_definitions_updated_at = Some(Utc::now())
    }

    pub(super) fn prices_updated_at(&self) -> Option<DateTime<Utc>> {
        self.prices_updated_at
    }

    pub(super) fn refresh_prices_updated_at_timestamp(&mut self) {
        self.prices_updated_at = Some(Utc::now())
    }

    /// Check if the price data is up to date.
    /// it's outdated if it hasn't been updated since 4 AM UTC
    pub fn is_price_updated(&self) -> bool {
        let prices_updated_at: DateTime<Utc> = match self.prices_updated_at {
            Some(date) => date,
            None => return false,
        };

        let utc_now = Utc::now();
        // Set 4 AM for the current day
        let utc_4am = DateTime::<Utc>::from_naive_utc_and_offset(
            NaiveDateTime::new(
                utc_now.date_naive(),
                NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
            ),
            Utc,
        );

        if utc_now < utc_4am {
            // If current time is before 4 AM, check if prices were updated yesterday
            let utc_4am_yesterday = DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDateTime::new(
                    utc_4am.date_naive().pred_opt().unwrap(),
                    NaiveTime::from_hms_opt(4, 0, 0).unwrap(),
                ),
                Utc,
            );

            return prices_updated_at > utc_4am_yesterday;
        }

        // Otherwise, check if prices were updated today after 4 AM
        prices_updated_at > utc_4am
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use testresult::TestResult;

    #[test]
    fn test_goatbots_metadata() -> TestResult {
        let card_defs_updated_at_datetime: DateTime<Utc> = "2023-10-21T22:29:53Z".parse()?;
        let prices_updated_at_datetime: DateTime<Utc> = "2023-10-14T15:24:21Z".parse()?;

        let goatbots_metadata = GoatBotsMetaData {
            card_definitions_updated_at: Some(card_defs_updated_at_datetime),
            prices_updated_at: Some(prices_updated_at_datetime),
        };

        let serialized = toml::to_string(&goatbots_metadata)?;

        eprintln!("{serialized}");

        assert_str_eq!(
            serialized,
            "\
card_definitions_updated_at = \"2023-10-21T22:29:53Z\"
prices_updated_at = \"2023-10-14T15:24:21Z\"
"
        );

        Ok(())
    }

    #[test]
    fn test_is_prices_updated_true() -> TestResult {
        let goatbots_metadata = GoatBotsMetaData {
            card_definitions_updated_at: Some("2023-10-21T22:29:53Z".parse()?),
            prices_updated_at: Some(Utc::now()),
        };

        assert!(goatbots_metadata.is_price_updated());
        Ok(())
    }

    #[test]
    fn test_is_prices_updated_false() -> TestResult {
        let goatbots_metadata = GoatBotsMetaData {
            card_definitions_updated_at: Some("2023-10-21T22:29:53Z".parse()?),
            prices_updated_at: Some(Utc.with_ymd_and_hms(2023, 10, 1, 0, 1, 1).unwrap()),
        };

        assert_eq!(goatbots_metadata.is_price_updated(), false);
        Ok(())
    }
}
