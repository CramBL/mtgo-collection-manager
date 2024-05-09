use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};

mod goatbots_md;
use goatbots_md::GoatBotsMetaData;

mod scryfall_md;
use scryfall_md::ScryfallMetaData;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInfoMetaData {
    description: &'static str,
    goatbots: GoatBotsMetaData,
    scryfall: ScryfallMetaData,
}

impl CardInfoMetaData {
    const DESCRIPTION: &'static str = "log for MTGO Getter state, such as updated_at timestamps";

    /// Create a first time [CardInfoMetaData] state log.
    /// From then on it should always be edited/updated on disk instead of creating a new one.
    pub fn new() -> Self {
        Self {
            description: Self::DESCRIPTION,
            goatbots: GoatBotsMetaData::default(),
            scryfall: ScryfallMetaData::default(),
        }
    }

    /// Check if the price data is up to date. it's outdated if it hasn't been updated since 4 AM UTC
    pub fn is_goatbots_prices_updated(&self) -> bool {
        self.goatbots.is_price_updated()
    }

    /// Check if the card definitions are up to date.
    ///
    /// it's updated unless a new set has been released and it's been >20 minutes since last update
    pub fn is_card_definitions_updated(&self) -> bool {
        let card_definitions_updated_at: DateTime<Utc> =
            match self.goatbots.card_definitions_updated_at() {
                Some(date) => date,
                None => return false, // They were never updated
            };

        if self.scryfall.next_released_mtgo_set().is_any_none() {
            return false; // There's no next set, assume they need to be updated (were never updated in the first place)
        }

        let twenty_minutes_ago = Utc::now() - chrono::Duration::minutes(20);
        let next_release_date = self
            .scryfall
            .next_released_mtgo_set()
            .released_at()
            .unwrap();

        // If the next release date is after the current date,
        //  or the card definitions were updated less than 20 min. ago
        //  then consider card definitions to be updated
        (next_release_date >= Utc::now().date_naive())
            || (card_definitions_updated_at < twenty_minutes_ago)
    }

    /// Check if the bulk data is up to date.
    /// outdated if the timestamp is older than the `updated_at` retrieved from the Scryfall API
    pub fn is_scryfall_bulk_updated(&self, api_timestamp: DateTime<Utc>) -> bool {
        self.scryfall
            .bulk_data_updated_at()
            .is_some_and(|dt| dt > api_timestamp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use scryfall_md::next_released_mtgo_set::NextReleasedMtgoSet;
    use testresult::TestResult;

    #[test]
    fn test_cardinfo_metadata() -> TestResult {
        let next_released_mtgo_set = NextReleasedMtgoSet::new(
            Some("The Lost Caverns of Ixalan".to_string()),
            Some(NaiveDate::from_ymd_opt(2023, 12, 11).unwrap()),
            Some("lci".to_string()),
        );
        let scryfall_metadata = ScryfallMetaData::new(
            Some("1970-01-01T00:00:00Z".parse()?),
            next_released_mtgo_set,
        );
        let goatbots_metadata = GoatBotsMetaData::new(
            Some("2023-10-21T22:29:53Z".parse()?),
            Some("2023-10-14T15:24:21Z".parse()?),
        );

        let cardinfo_metadata = CardInfoMetaData {
            description: CardInfoMetaData::DESCRIPTION,
            goatbots: goatbots_metadata,
            scryfall: scryfall_metadata,
        };

        let serialized = toml::to_string(&cardinfo_metadata)?;

        eprintln!("{serialized}");

        assert_str_eq!(
            serialized,
            "\
[goatbots]
card_definitions_updated_at = \"2023-10-21T22:29:53Z\"
prices_updated_at = \"2023-10-14T15:24:21Z\"

[scryfall]
bulk_data_updated_at = \"1970-01-01T00:00:00Z\"

[scryfall.next_released_mtgo_set]
name = \"The Lost Caverns of Ixalan\"
released_at = \"2023-12-11\"
mtgo_code = \"lci\"
"
        );

        Ok(())
    }

    /// Should be false cause no data was ever fetched
    #[test]
    fn test_is_scryfall_bulk_updated_false_cause_empty() -> TestResult {
        let cardinfo_metadata = CardInfoMetaData {
            description: CardInfoMetaData::DESCRIPTION,
            goatbots: GoatBotsMetaData::default(),
            scryfall: ScryfallMetaData::default(),
        };

        let updated = cardinfo_metadata.is_scryfall_bulk_updated(DateTime::<Utc>::default());

        assert_eq!(updated, false);

        Ok(())
    }

    /// Should be true because the local timestamp is newer than the one from the Scryfall API.
    #[test]
    fn test_is_scryfall_bulk_updated_true() -> TestResult {
        let cardinfo_metadata = CardInfoMetaData {
            description: CardInfoMetaData::DESCRIPTION,
            goatbots: GoatBotsMetaData::default(),
            scryfall: ScryfallMetaData::new(Some(Utc::now()), NextReleasedMtgoSet::default()),
        };

        let fake_scryfall_api_timestamp = Utc::now() - chrono::Duration::days(1);
        let updated = cardinfo_metadata.is_scryfall_bulk_updated(fake_scryfall_api_timestamp);

        assert_eq!(updated, true);

        Ok(())
    }

    /// Should be false because local timestamp is older than the timestamp from the API.
    #[test]
    fn test_is_scryfall_bulk_updated_false() -> TestResult {
        let cardinfo_metadata = CardInfoMetaData {
            description: CardInfoMetaData::DESCRIPTION,
            goatbots: GoatBotsMetaData::default(),
            scryfall: ScryfallMetaData::new(
                Some(Utc::now() - chrono::Duration::days(1)),
                NextReleasedMtgoSet::default(),
            ),
        };

        let fake_scryfall_api_timestamp = Utc::now();
        let updated = cardinfo_metadata.is_scryfall_bulk_updated(fake_scryfall_api_timestamp);

        assert_eq!(updated, false);

        Ok(())
    }
}
