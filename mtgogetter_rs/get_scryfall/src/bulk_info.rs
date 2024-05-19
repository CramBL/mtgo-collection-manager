use chrono::{DateTime, Utc};
use reqwest::{blocking, Error, Url};
use serde::{Deserialize, Serialize};
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
    use regex::Regex;
    use testresult::TestResult;

    use super::*;

    #[ignore = "Will download data from the Scryfall API"]
    #[test]
    fn test_get_scryfall_bulk_info() -> TestResult {
        let scryfall_bulk_info = ScryfallBulkDataInfo::get()?;

        eprintln!("{scryfall_bulk_info:?}");
        eprintln!("URL: {}", scryfall_bulk_info.download_url().as_str());
        eprintln!("Updated at: {}", scryfall_bulk_info.updated_at());

        assert!(scryfall_bulk_info
            .download_url()
            .as_str()
            .starts_with("https://data.scryfall.io/default-cards/default-cards-"));
        let re = Regex::new("https://data.scryfall.io/default-cards/default-cards-[0-9]{14}.json")
            .unwrap();
        assert!(re.is_match(scryfall_bulk_info.download_url().as_str()));

        let re_date =
            Regex::new("[0-9]{4}-[0-9]{2}-[0-9]{2} [0-9]{2}:[0-9]{2}:[0-9]{2}.[0-9]{3} UTC")
                .unwrap();
        assert!(re_date.is_match(&scryfall_bulk_info.updated_at().to_string()));

        Ok(())
    }
}
