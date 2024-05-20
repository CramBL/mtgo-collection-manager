use std::{error::Error, fmt, io::BufReader};

use chrono::{NaiveDate, Utc};
use reqwest::blocking;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct ApiResponseScryfallSet {
    data: Vec<Set>,
}

#[derive(Deserialize, Debug, Clone)]
struct Set {
    name: String,
    released_at: String,
    mtgo_code: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MtgoSet {
    pub name: String,
    pub released_at: NaiveDate,
    pub mtgo_code: String,
}

impl MtgoSet {
    fn from_set(set: Set) -> Option<Self> {
        let mtgo_code = set.mtgo_code?;
        let released_at: NaiveDate =
            NaiveDate::parse_from_str(&set.released_at, "%Y-%m-%d").unwrap();

        Some(Self {
            name: set.name,
            released_at,
            mtgo_code,
        })
    }
}

pub struct ScryfallMtgoSets {
    mtgo_sets: Vec<MtgoSet>,
}

impl ScryfallMtgoSets {
    pub const FILENAME: &'static str = "scryfall-sets.json";
    const SCRYFALL_SET_LIST_URL: &'static str = "https://api.scryfall.com/sets";

    pub fn get() -> Result<Self, reqwest::Error> {
        let resp = blocking::get(Self::SCRYFALL_SET_LIST_URL)?;
        let stream = BufReader::new(resp);
        let ApiResponseScryfallSet { data } = serde_json::from_reader(stream).unwrap();
        let sets: Vec<Set> = data;
        let mtgo_sets: Vec<MtgoSet> = sets.into_iter().filter_map(MtgoSet::from_set).collect();

        Ok(Self { mtgo_sets })
    }

    pub fn next_released_mtgo_set(&self) -> Result<Option<&MtgoSet>, Box<dyn Error>> {
        let now = Utc::now().date_naive();
        next_released_mtgo_set(now, &self.mtgo_sets)
    }

    /// Returns a vector of [MtgoSet] consuming the [ScryfallMtgoSets] in the process.
    pub fn take_sets(self) -> Vec<MtgoSet> {
        self.mtgo_sets
    }
}

/// Takes a [NaiveDate] and a slice of [MtgoSet] and returns a reference to the
/// [MtgoSet] that will be released next (first match, more than one could be released at the same date).
///
/// # Errors
///
/// If none of the [MtgoSet]s are released after the given date.
pub fn next_released_mtgo_set(
    target_time: NaiveDate,
    sets: &[MtgoSet],
) -> Result<Option<&MtgoSet>, Box<dyn Error>> {
    let mut next_released_set: Option<&MtgoSet> = None;

    // Go back at most 100 sets
    for set in sets.iter().take(100) {
        if set.released_at < target_time {
            if next_released_set.is_some() {
                break;
            }
            return Err(Box::new(fmt::Error)); // Use a standard error
        }
        next_released_set = Some(set);
    }

    Ok(next_released_set)
}

#[cfg(test)]
mod tests {
    use super::*;
    use testresult::TestResult;

    static RAW_JSON_TEST_DATA: &[u8] =
        include_bytes!("../../../test/test-data/scryfall/sets-small-16sets.json");
    static JSON_TEST_DATA_STR: &str =
        include_str!("../../../test/test-data/scryfall/sets-small-16sets.json");

    #[test]
    fn test_url_is_ok() {
        assert!(reqwest::Url::parse(ScryfallMtgoSets::SCRYFALL_SET_LIST_URL).is_ok());
    }

    #[ignore = "Will download data from the Scryfall API"]
    #[test]
    fn test_get_scryfall_sets() -> TestResult {
        let sets = ScryfallMtgoSets::get().unwrap();
        let sets = sets.take_sets();

        eprintln!("{sets:?}");

        assert!(serde_json::to_string(&sets).is_ok());

        Ok(())
    }

    #[test]
    fn test_parse_scryfall_set_json() -> TestResult {
        let json = r#"{
            "object": "list",
            "has_more": false,
            "data": [
              {
                "object": "set",
                "id": "fed2c8cd-ab92-44f6-808a-41e7809bcfe2",
                "code": "rvr",
                "tcgplayer_id": 23319,
                "name": "Ravnica Remastered",
                "uri": "https://api.scryfall.com/sets/fed2c8cd-ab92-44f6-808a-41e7809bcfe2",
                "scryfall_uri": "https://scryfall.com/sets/rvr",
                "search_uri": "https://api.scryfall.com/cards/search?include_extras=true&include_variations=true&order=set&q=e%3Arvr&unique=prints",
                "released_at": "2024-03-01",
                "set_type": "masters",
                "card_count": 50,
                "digital": false,
                "nonfoil_only": false,
                "foil_only": false,
                "icon_svg_uri": "https://svgs.scryfall.io/sets/rvr.svg?1696824000"
              }
            ]
          }"#;

        let response: ApiResponseScryfallSet = serde_json::from_str(json)?;
        assert!(response.data[0].mtgo_code.is_none());
        assert_eq!(response.data[0].released_at, "2024-03-01");
        Ok(())
    }

    /// Should find the bottom set of the json-file as the target date is before any of the sets in the file
    #[test]
    fn test_parse_from_test_data_file() -> TestResult {
        let response: ApiResponseScryfallSet = serde_json::from_str(JSON_TEST_DATA_STR)?;

        let mtgo_sets: Vec<MtgoSet> = response
            .data
            .into_iter()
            .filter_map(MtgoSet::from_set)
            .collect();

        let target_time = NaiveDate::from_ymd_opt(2023, 5, 1).unwrap();

        let next_released_mtgo_set = next_released_mtgo_set(target_time, &mtgo_sets);

        let res = next_released_mtgo_set.unwrap().unwrap();
        assert_eq!(res.name, "Wilds of Eldraine Tokens");
        assert_eq!(
            res.released_at,
            NaiveDate::from_ymd_opt(2023, 9, 8).unwrap()
        );
        assert_eq!(res.mtgo_code, "twoe");

        Ok(())
    }

    #[test]
    fn test_parse_from_test_data_file_newer() -> TestResult {
        let response: ApiResponseScryfallSet = serde_json::from_slice(RAW_JSON_TEST_DATA)?;
        let mtgo_sets: Vec<MtgoSet> = response
            .data
            .into_iter()
            .filter_map(MtgoSet::from_set)
            .collect();
        let target_time = NaiveDate::from_ymd_opt(2023, 10, 14).unwrap();

        let next_released_mtgo_set = next_released_mtgo_set(target_time, &mtgo_sets);

        let res = next_released_mtgo_set.unwrap().unwrap();
        assert_eq!(res.name, "Lost Caverns of Ixalan Commander");
        assert_eq!(
            res.released_at,
            NaiveDate::from_ymd_opt(2023, 11, 17).unwrap()
        );
        assert_eq!(res.mtgo_code, "lcc");

        Ok(())
    }

    /// Tests the scenario where the date is exactly equal to a set's release
    /// it should be the same behaviour as if the date was just before that set's release
    #[test]
    fn test_next_released_scryfall_set_from_json_bytes_exact_date() -> TestResult {
        let response: ApiResponseScryfallSet = serde_json::from_slice(RAW_JSON_TEST_DATA)?;
        let mtgo_sets: Vec<MtgoSet> = response
            .data
            .into_iter()
            .filter_map(MtgoSet::from_set)
            .collect();

        let target_time = NaiveDate::from_ymd_opt(2023, 11, 17).unwrap();

        let next_released_mtgo_set = next_released_mtgo_set(target_time, &mtgo_sets);

        let res = next_released_mtgo_set.unwrap().unwrap();
        assert_eq!(res.name, "Lost Caverns of Ixalan Commander");
        assert_eq!(
            res.released_at,
            NaiveDate::from_ymd_opt(2023, 11, 17,).unwrap()
        );
        assert_eq!(res.mtgo_code, "lcc");

        Ok(())
    }

    /// Tests where the target date is later than the latest set release, meaning we won't find a set that will be released in the future
    /// this should never happen and should error
    #[test]
    fn test_next_released_scryfall_set_from_json_bytes_later_date_errors() -> TestResult {
        let response: ApiResponseScryfallSet = serde_json::from_slice(RAW_JSON_TEST_DATA)?;
        let mtgo_sets: Vec<MtgoSet> = response
            .data
            .into_iter()
            .filter_map(MtgoSet::from_set)
            .collect();

        let target_time = NaiveDate::from_ymd_opt(2024, 11, 17).unwrap();

        let next_released_mtgo_set = next_released_mtgo_set(target_time, &mtgo_sets);

        assert!(next_released_mtgo_set.is_err());
        Ok(())
    }
}
