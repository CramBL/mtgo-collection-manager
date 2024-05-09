use chrono::{DateTime, NaiveDate, Utc};
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInfoMetaData {
    goatbots: GoatBotsMetaData,
    scryfall: ScryfallMetaData,
}

impl CardInfoMetaData {
    pub fn goatbots_card_definitions_updated_at(&self) -> DateTime<Utc> {
        self.goatbots.card_definitions_updated_at
    }

    pub fn goatbots_prices_updated_at(&self) -> DateTime<Utc> {
        self.goatbots.prices_updated_at
    }

    pub fn scryfall_bulk_data_updated_at(&self) -> DateTime<Utc> {
        self.scryfall.bulk_data_updated_at
    }

    pub fn scryfall_next_released_mtgo_set(&self) -> &NextReleasedMtgoSet {
        &self.scryfall.next_released_mtgo_set
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct GoatBotsMetaData {
    card_definitions_updated_at: DateTime<Utc>,
    prices_updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
struct ScryfallMetaData {
    bulk_data_updated_at: DateTime<Utc>,
    next_released_mtgo_set: NextReleasedMtgoSet,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextReleasedMtgoSet {
    name: String,
    released_at: NaiveDate,
    mtgo_code: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use testresult::TestResult;

    #[test]
    fn test_next_released_mtgoset() -> TestResult {
        let naive_date = NaiveDate::from_ymd_opt(2023, 12, 11).unwrap();
        let next_released_mtgo_set = NextReleasedMtgoSet {
            name: "The Lost Caverns of Ixalan".to_string(),
            released_at: naive_date,
            mtgo_code: "lci".to_string(),
        };

        let toml_str = toml::to_string(&next_released_mtgo_set)?;
        assert_str_eq!(
            toml_str,
            "\
            name = \"The Lost Caverns of Ixalan\"\n\
            released_at = \"2023-12-11\"\n\
            mtgo_code = \"lci\"\n\
            ",
        );
        let from_toml_str = toml::from_str(&toml_str)?;
        assert_eq!(next_released_mtgo_set, from_toml_str);

        Ok(())
    }

    #[test]
    fn test_scryfall_metadata() -> TestResult {
        let next_released_mtgo_set = NextReleasedMtgoSet {
            name: "The Lost Caverns of Ixalan".to_string(),
            released_at: NaiveDate::from_ymd_opt(2023, 12, 11).unwrap(),
            mtgo_code: "lci".to_string(),
        };

        let updated_at_datetime: DateTime<Utc> = "1970-01-01T00:00:00Z".parse()?;
        let scryfall_md = ScryfallMetaData {
            bulk_data_updated_at: updated_at_datetime,
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

    #[test]
    fn test_goatbots_metadata() -> TestResult {
        let card_defs_updated_at_datetime: DateTime<Utc> = "2023-10-21T22:29:53Z".parse()?;
        let prices_updated_at_datetime: DateTime<Utc> = "2023-10-14T15:24:21Z".parse()?;

        let goatbots_metadata = GoatBotsMetaData {
            card_definitions_updated_at: card_defs_updated_at_datetime,
            prices_updated_at: prices_updated_at_datetime,
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
    fn test_cardinfo_metadata() -> TestResult {
        let next_released_mtgo_set = NextReleasedMtgoSet {
            name: "The Lost Caverns of Ixalan".to_string(),
            released_at: NaiveDate::from_ymd_opt(2023, 12, 11).unwrap(),
            mtgo_code: "lci".to_string(),
        };
        let scryfall_metadata = ScryfallMetaData {
            bulk_data_updated_at: "1970-01-01T00:00:00Z".parse()?,
            next_released_mtgo_set,
        };
        let goatbots_metadata = GoatBotsMetaData {
            card_definitions_updated_at: "2023-10-21T22:29:53Z".parse()?,
            prices_updated_at: "2023-10-14T15:24:21Z".parse()?,
        };

        let cardinfo_metadata = CardInfoMetaData {
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
}
