use std::{fs, io, path::Path};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod goatbots_md;
use goatbots_md::GoatBotsMetaData;

pub(crate) mod scryfall_md;
use scryfall_md::ScryfallMetaData;

use self::scryfall_md::next_released_mtgo_set::NextReleasedMtgoSet;

/// Also known as `fetch_log.toml`
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInfoMetaData {
    description: Box<str>,
    goatbots: GoatBotsMetaData,
    scryfall: ScryfallMetaData,
}

impl CardInfoMetaData {
    pub const FILENAME: &'static str = "fetch_log.toml";
    const DESCRIPTION: &'static str = "log for MTGO Getter state, such as updated_at timestamps";

    /// Create a first time [CardInfoMetaData] state log.
    /// From then on it should always be edited/updated on disk instead of creating a new one.
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            description: Self::DESCRIPTION.into(),
            goatbots: GoatBotsMetaData::default(),
            scryfall: ScryfallMetaData::default(),
        }
    }

    pub fn from_toml_file(p: &Path) -> Result<Self, io::Error> {
        let contents = fs::read_to_string(p)?;
        // TODO: Handle/converge error types here (thiserror?)
        let toml: CardInfoMetaData = toml::from_str(&contents).unwrap();
        Ok(toml)
    }

    pub fn to_toml_on_disk(&self, p: &Path) -> Result<(), io::Error> {
        let toml = toml::to_string(self).unwrap();
        fs::write(p, toml)
    }

    /// Check if the price data is up to date. it's outdated if it hasn't been updated since 4 AM UTC
    pub fn is_goatbots_prices_updated(&self) -> bool {
        self.goatbots.is_price_updated()
    }

    /// Refresh the timestamp by assigning the current UTC time.
    pub fn refresh_prices_updated_at_timestamp(&mut self) {
        self.goatbots.refresh_prices_updated_at_timestamp()
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

        if self.scryfall.is_next_set_out() {
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

    /// Refresh the timestamp by assigning the current UTC time.
    pub fn refresh_card_definitions_updated_at_timestamp(&mut self) {
        self.goatbots
            .refresh_card_definitions_updated_at_timestamp();
    }

    /// Check if the bulk data is up to date.
    /// outdated if the timestamp is older than the `updated_at` retrieved from the Scryfall API
    pub fn is_scryfall_bulk_updated(&self, api_timestamp: DateTime<Utc>) -> bool {
        self.scryfall
            .bulk_data_updated_at()
            .is_some_and(|dt| dt > api_timestamp)
    }

    /// Refresh the timestamp by assigning the current UTC time.
    pub fn refresh_bulk_data_updated_at_timestamp(&mut self) {
        self.scryfall.refresh_bulk_data_updated_at_timestamp();
    }

    // Returns if the next set to come out is now out on MTGO.
    //
    // If it is out, we want to update which set is the next to come out
    pub fn is_next_set_out(&self) -> bool {
        self.scryfall.is_next_set_out()
    }

    /// Replace the [NextReleasedMtgoSet] with the given set
    pub fn replace_next_released_set(&mut self, next_set: NextReleasedMtgoSet) {
        self.scryfall.replace_next_released_set(next_set);
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
            description: CardInfoMetaData::DESCRIPTION.into(),
            goatbots: goatbots_metadata,
            scryfall: scryfall_metadata,
        };

        let serialized = toml::to_string(&cardinfo_metadata)?;

        eprintln!("{serialized}");

        assert_str_eq!(
            serialized,
            "\
description = \"log for MTGO Getter state, such as updated_at timestamps\"

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

        let deserialized: CardInfoMetaData = toml::from_str(&serialized)?;
        assert_eq!(cardinfo_metadata, deserialized);

        Ok(())
    }

    /// Should be false cause no data was ever fetched
    #[test]
    fn test_is_scryfall_bulk_updated_false_cause_empty() -> TestResult {
        let cardinfo_metadata = CardInfoMetaData {
            description: CardInfoMetaData::DESCRIPTION.into(),
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
            description: CardInfoMetaData::DESCRIPTION.into(),
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
            description: CardInfoMetaData::DESCRIPTION.into(),
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
