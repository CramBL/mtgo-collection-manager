use chrono::{DateTime, Utc};
use reqwest::{blocking, Url};
use serde::{Deserialize, Serialize};


use crate::util::format_datetime_utc_for_url;

#[derive(Debug)]
pub struct ScryfallBulkData {
    endpoint: Url,
    updated_at: DateTime<Utc>,
}

impl ScryfallBulkData {
    const URL_PREFIX: &'static str = "https://data.scryfall.io/default-cards/default-cards-";

    /// Get bulk data from the supplied date
    pub fn get(date: DateTime<Utc>) -> Result<Self, reqwest::Error> {
        let url_str = String::from(Self::URL_PREFIX) + &format_datetime_utc_for_url(date) + ".json";
        let url_endpoint = Url::parse(&url_str).unwrap();
        let resp = blocking::get(url_endpoint.clone())?;

        let bytes = resp.bytes()?;

        #[derive(Deserialize, Serialize, Debug)]
        struct TmpBulkDataInfo {
            updated_at: DateTime<Utc>,
        }

        let deserialized: TmpBulkDataInfo = serde_json::from_slice(&bytes).unwrap();

        let bulk_data_info: Self = Self {
            endpoint: url_endpoint,
            updated_at: deserialized.updated_at,
        };

        Ok(bulk_data_info)
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }
}

#[cfg(test)]
mod tests {
    use testresult::TestResult;

    use super::*;

    #[ignore = "Will download data from the Scryfall API"]
    #[test]
    fn test_get_scryfall_bulk_info() -> TestResult {
        // let scryfall_bulk_info = ScryfallBulkData::get()?;

        // eprintln!("{scryfall_bulk_info:?}");
        Ok(())
    }
}
