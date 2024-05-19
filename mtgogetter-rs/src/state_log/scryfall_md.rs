#![allow(dead_code)]
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use self::next_released_mtgo_set::NextReleasedMtgoSet;

pub mod next_released_mtgo_set;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScryfallMetaData {
    bulk_data_updated_at: Option<DateTime<Utc>>,
    next_released_mtgo_set: NextReleasedMtgoSet,
}

impl ScryfallMetaData {
    pub fn new(
        bulk_data_updated_at: Option<DateTime<Utc>>,
        next_released_mtgo_set: NextReleasedMtgoSet,
    ) -> Self {
        Self {
            bulk_data_updated_at,
            next_released_mtgo_set,
        }
    }

    /// The updated at timestamp is equal to the last time the Scryfall API was queried for bulk data.
    pub fn bulk_data_updated_at(&self) -> Option<DateTime<Utc>> {
        self.bulk_data_updated_at
    }

    pub fn next_released_mtgo_set(&self) -> &NextReleasedMtgoSet {
        &self.next_released_mtgo_set
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use pretty_assertions::assert_str_eq;
    use testresult::TestResult;

    #[test]
    fn test_scryfall_metadata() -> TestResult {
        let next_released_mtgo_set = NextReleasedMtgoSet::new(
            Some("The Lost Caverns of Ixalan".to_string()),
            Some(NaiveDate::from_ymd_opt(2023, 12, 11).unwrap()),
            Some("lci".to_string()),
        );

        let updated_at_datetime: DateTime<Utc> = "1970-01-01T00:00:00Z".parse()?;
        let scryfall_md = ScryfallMetaData {
            bulk_data_updated_at: Some(updated_at_datetime),
            next_released_mtgo_set,
        };

        let serialized = toml::to_string(&scryfall_md)?;
        eprintln!("{serialized}");

        assert_str_eq!(
            serialized,
            "\
bulk_data_updated_at = \"1970-01-01T00:00:00Z\"

[next_released_mtgo_set]
name = \"The Lost Caverns of Ixalan\"
released_at = \"2023-12-11\"
mtgo_code = \"lci\"
",
        );

        Ok(())
    }
}
