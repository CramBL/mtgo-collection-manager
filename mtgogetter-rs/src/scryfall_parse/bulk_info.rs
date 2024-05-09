use chrono::{DateTime, Utc};
use reqwest::{blocking, Error, Url};
use serde_derive::{Deserialize, Serialize};
use std::{fs, io::Cursor};

#[derive(Debug)]
pub struct ScryfallBulkDataInfo {
    download_url: Url,
    updated_at: DateTime<Utc>,
}

impl ScryfallBulkDataInfo {
    const URL_ENDPOINT: &'static str =
        "https://api.scryfall.com/bulk-data/e2ef41e3-5778-4bc2-af3f-78eca4dd9c23";

    pub fn get() -> Result<Self, reqwest::Error> {
        let resp = blocking::get(Self::URL_ENDPOINT)?;
        let bytes = resp.bytes()?;

        #[derive(Deserialize, Serialize, Debug)]
        struct TmpBulkDataInfo<'b> {
            download_uri: &'b str,
            updated_at: DateTime<Utc>,
        }

        let deserialized: TmpBulkDataInfo = serde_json::from_slice(&bytes).unwrap();
        let download_url = Url::parse(deserialized.download_uri).unwrap();
        let bulk_data_info: Self = Self {
            download_url,
            updated_at: deserialized.updated_at,
        };

        Ok(bulk_data_info)
    }

    pub fn download_url(&self) -> &Url {
        &self.download_url
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
        let scryfall_bulk_info = ScryfallBulkDataInfo::get()?;

        eprintln!("{scryfall_bulk_info:?}");
        Ok(())
    }
}
