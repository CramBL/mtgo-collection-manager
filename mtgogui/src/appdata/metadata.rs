//! Contains the [MetaData] struct and substructs for storing the metadata of the MTGO Getter state log.
//!
//! It is implemented with Regex parsing because the TOML parser is not able to parse the state log file.
//! Due to Go's `time.Time` being stored as a raw value in the TOML file and not as a quoted string.

// Author notes: Implementing a custom serializer/deserializer for Serde is not feasible as Go's `time.Time` is not valid TOML.
//               Implementing a custom visitor for Serde has the same problem.
// Implementing the parsing with Regex is feasible and also pretty simple
// The regex parsing assumes:
// - The field names are an exact match
// - All field names are present and they are unique (this can be laxed with a more complex regex if required)

use super::MTGO_GETTER_STATE_LOG;
use chrono::{DateTime, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{io, path::PathBuf};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MetaData {
    goatbots: GoatBots,
    scryfall: Scryfall,
}

impl MetaData {
    /// Load the metadata from the MTGO Getter state log
    ///
    /// # Arguments
    ///
    /// * `src_dir` - The directory where the MTGO Getter state log is located
    ///
    /// # Errors
    ///
    /// Returns an error if the MTGO Getter state log is not found or if the parsing fails
    pub fn load(mut src_dir: PathBuf) -> io::Result<Self> {
        src_dir.push("fetch_log.toml");
        let gui_state = if src_dir.try_exists()? {
            let toml = std::fs::read_to_string(src_dir)?;
            let goatbots = match GoatBots::from_raw_string(&toml) {
                Ok(goatbots) => goatbots,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Error getting Goatbots metadata: {e}"),
                    ))
                }
            };
            let scryfall = match Scryfall::from_raw_string(&toml) {
                Ok(scryfall) => scryfall,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Error getting Scryfall metadata: {e}"),
                    ))
                }
            };
            Self { goatbots, scryfall }
        } else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No MTGO Getter state log TOML found",
            ));
        };
        Ok(gui_state)
    }

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

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct GoatBots {
    card_definitions_updated_at: DateTime<Utc>,
    prices_updated_at: DateTime<Utc>,
}

impl GoatBots {
    // Regex for extracting the fields from a raw string
    const RE: &'static str = r"card_definitions_updated_at = (?<card_definitions_updated_at>.*Z).*\n.*prices_updated_at = (?<prices_updated_at>.*)";

    pub fn from_raw_string(raw: &str) -> Result<Self, regex::Error> {
        // Make sure the regex is only compiled once
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(GoatBots::RE).expect("Failed to compile regex"));

        let caps = match RE.captures(raw) {
            Some(caps) => caps,
            None => {
                return Err(regex::Error::Syntax(
                    "Failed to parse string for Goatbots metadata".to_string(),
                ))
            }
        };

        let card_definitions_updated_at =
            if let Some(caps) = caps.name("card_definitions_updated_at") {
                match caps.as_str().parse::<DateTime<Utc>>() {
                    Ok(card_definitions_updated_at) => card_definitions_updated_at,
                    Err(e) => {
                        return Err(regex::Error::Syntax(format!(
                        "Failed to parse card_definitions_updated_at match to DateTime<Utc>: {e}"
                    )));
                    }
                }
            } else {
                return Err(regex::Error::Syntax(
                    "Failed to parse card_definitions_updated_at, no match found".into(),
                ));
            };

        let prices_updated_at = if let Some(caps) = caps.name("prices_updated_at") {
            match caps.as_str().parse::<DateTime<Utc>>() {
                Ok(prices_updated_at) => prices_updated_at,
                Err(e) => {
                    return Err(regex::Error::Syntax(format!(
                        "Failed to parse prices_updated_at match to DateTime<Utc>: {e}"
                    )));
                }
            }
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse prices_updated_at, no match found".into(),
            ));
        };

        Ok(Self {
            card_definitions_updated_at,
            prices_updated_at,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct Scryfall {
    bulk_data_updated_at: DateTime<Utc>,
    next_released_mtgo_set: NextReleasedMtgoSet,
}

impl Scryfall {
    const RE: &'static str = r"bulk_data_updated_at = (?<bulk_data_updated_at>.*Z)";

    pub fn from_raw_string(raw: &str) -> Result<Self, regex::Error> {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(Scryfall::RE).expect("Failed to compile regex"));

        let caps = if let Some(caps) = RE.captures(raw) {
            caps
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse string for Scryfall metadata".to_string(),
            ));
        };

        let bulk_data_updated_at = if let Some(caps) = caps.name("bulk_data_updated_at") {
            match caps.as_str().parse::<DateTime<Utc>>() {
                Ok(bulk_data_updated_at) => bulk_data_updated_at,
                Err(e) => {
                    return Err(regex::Error::Syntax(format!(
                        "Failed to parse bulk_data_updated_at match to DateTime<Utc>: {e}"
                    )));
                }
            }
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse bulk_data_updated_at, no match found".into(),
            ));
        };

        let next_released_mtgo_set = NextReleasedMtgoSet::from_raw_string(raw)?;

        Ok(Self {
            bulk_data_updated_at,
            next_released_mtgo_set,
        })
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct NextReleasedMtgoSet {
    name: String,
    released_at: String, // This can be changed to DateTime<Utc> as well if required
    mtgo_code: String,
}

impl NextReleasedMtgoSet {
    // Regex for extracting the fields from a raw string
    const RE: &'static str = r#"name = "(?<name>.*)".*\n.*released_at = "(?<released_at>.*)".*\n.*mtgo_code = "(?<mtgo_code>.*)""#;

    pub fn from_raw_string(raw: &str) -> Result<Self, regex::Error> {
        // Make sure the regex is only compiled once
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(NextReleasedMtgoSet::RE).expect("Failed to compile regex"));

        let caps = if let Some(caps) = RE.captures(raw) {
            caps
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse string for NextReleasedMtgoSet metadata".to_string(),
            ));
        };

        let name = if let Some(caps) = caps.name("name") {
            caps.as_str().to_string()
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse name, no match found".into(),
            ));
        };

        let released_at = if let Some(caps) = caps.name("released_at") {
            caps.as_str().to_string()
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse released_at, no match found".into(),
            ));
        };

        let mtgo_code = if let Some(caps) = caps.name("mtgo_code") {
            caps.as_str().to_string()
        } else {
            return Err(regex::Error::Syntax(
                "Failed to parse mtgo_code, no match found".into(),
            ));
        };

        Ok(Self {
            name,
            released_at,
            mtgo_code,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use temp_dir::TempDir;

    const TEST_STATE_LOG: &str =
        include_str!("../../../test/test-data/mtgogetter-out/fetch_log.toml");

    #[test]
    fn test_metadata_goatbots_load_ok() {
        eprintln!("{TEST_STATE_LOG}");
        let metadata_loaded = GoatBots::from_raw_string(TEST_STATE_LOG);

        assert_eq!(
            metadata_loaded.unwrap(),
            GoatBots {
                card_definitions_updated_at: "2023-10-21T22:29:53Z"
                    .parse::<DateTime<Utc>>()
                    .unwrap(),
                prices_updated_at: "2023-10-14T15:24:21Z".parse::<DateTime<Utc>>().unwrap(),
            }
        );
    }

    #[test]
    fn test_metadata_goatbots_load_fail() {
        let broken_metadata = TEST_STATE_LOG
            .to_string()
            .replace("card_definitions_updated_at", "Card_definitions_updated_at");

        let metadata_loaded = GoatBots::from_raw_string(&broken_metadata);

        assert!(metadata_loaded.is_err());
        assert_eq!(
            metadata_loaded.unwrap_err().to_string(),
            "Failed to parse string for Goatbots metadata"
        );
    }

    #[test]
    fn test_metadata_next_released_set_load_ok() {
        let metadata_loaded = NextReleasedMtgoSet::from_raw_string(TEST_STATE_LOG);

        assert_eq!(
            metadata_loaded.unwrap(),
            NextReleasedMtgoSet {
                name: "The Lost Caverns of Ixalan".to_string(),
                released_at: "2023-12-11".to_string(),
                mtgo_code: "lci".to_string(),
            }
        );
    }

    #[test]
    fn test_metadata_next_released_set_load_fail() {
        let broken_metadata = TEST_STATE_LOG
            .to_string()
            .replace("name", "Name")
            .replace("released_at", "Released_at")
            .replace("mtgo_code", "Mtgo_code");

        let metadata_loaded = NextReleasedMtgoSet::from_raw_string(&broken_metadata);

        assert!(metadata_loaded.is_err());
        assert_eq!(
            metadata_loaded.unwrap_err().to_string(),
            "Failed to parse string for NextReleasedMtgoSet metadata"
        );
    }

    #[test]
    fn test_metadata_scryfall_load_ok() {
        let metadata_loaded = Scryfall::from_raw_string(TEST_STATE_LOG);

        assert_eq!(
            metadata_loaded.unwrap(),
            Scryfall {
                bulk_data_updated_at: "1970-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap(),
                next_released_mtgo_set: NextReleasedMtgoSet {
                    name: "The Lost Caverns of Ixalan".to_string(),
                    released_at: "2023-12-11".to_string(),
                    mtgo_code: "lci".to_string(),
                },
            }
        );
    }

    #[test]
    fn test_metadata_scryfall_load_fail() {
        let broken_metadata = TEST_STATE_LOG
            .to_string()
            .replace("bulk_data_updated_at", "Bulk_data_updated_at");

        let metadata_loaded = Scryfall::from_raw_string(&broken_metadata);

        assert!(metadata_loaded.is_err());
        assert_eq!(
            metadata_loaded.unwrap_err().to_string(),
            "Failed to parse string for Scryfall metadata"
        );
    }

    #[test]
    fn test_metadata_load_ok() {
        let src_dir = PathBuf::from("../test/test-data/mtgogetter-out");
        assert!(src_dir.exists());
        assert!(src_dir.join("fetch_log.toml").exists());
        let metadata_loaded = MetaData::load(src_dir).unwrap();

        assert_eq!(
            metadata_loaded,
            MetaData {
                goatbots: GoatBots {
                    card_definitions_updated_at: "2023-10-21T22:29:53Z"
                        .parse::<DateTime<Utc>>()
                        .unwrap(),
                    prices_updated_at: "2023-10-14T15:24:21Z".parse::<DateTime<Utc>>().unwrap(),
                },
                scryfall: Scryfall {
                    bulk_data_updated_at: "1970-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap(),
                    next_released_mtgo_set: NextReleasedMtgoSet {
                        name: "The Lost Caverns of Ixalan".to_string(),
                        released_at: "2023-12-11".to_string(),
                        mtgo_code: "lci".to_string(),
                    },
                },
            }
        );
    }

    #[test]
    fn test_metadata_load_fail() {
        let broken_metadata = TEST_STATE_LOG
            .to_string()
            .replace("card_definitions_updated_at", "Card_definitions_updated_at");
        let temp_dir = TempDir::new().unwrap();
        let state_log_path = temp_dir.path().join(MTGO_GETTER_STATE_LOG);

        std::fs::write(state_log_path, broken_metadata).unwrap();

        let metadata_loaded = MetaData::load(temp_dir.path().to_path_buf());

        assert!(metadata_loaded.is_err());
        assert_eq!(
            metadata_loaded.unwrap_err().to_string(),
            "Error getting Goatbots metadata: Failed to parse string for Goatbots metadata"
        );
    }
}
