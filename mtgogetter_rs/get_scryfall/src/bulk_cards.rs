use std::io::BufReader;

use crate::util::format_datetime_utc_for_url;
use chrono::{DateTime, Utc};
use parse_scryfall::{ScryfallCard, ScryfallMtgoCards};
use reqwest::{blocking, Url};

#[derive(Debug)]
pub struct ScryfallBulkData {
    endpoint: Url,
    updated_at: DateTime<Utc>,
    cards: Vec<ScryfallCard>,
}

impl ScryfallBulkData {
    const URL_PREFIX: &'static str = "https://data.scryfall.io/default-cards/default-cards-";

    /// Get bulk data from the supplied date
    pub fn get(date: DateTime<Utc>) -> Result<Self, reqwest::Error> {
        let url_str = String::from(Self::URL_PREFIX) + &format_datetime_utc_for_url(date) + ".json";
        log::info!("Getting bulk cards from {url_str}");
        let url_endpoint = Url::parse(&url_str).unwrap();
        let resp = blocking::get(url_endpoint.clone())?;

        let content_lenght: String = match resp.content_length() {
            Some(l) => l.to_string(),
            // Why could it be unknown? Read more: https://docs.rs/reqwest/latest/reqwest/blocking/struct.Response.html#method.content_length
            None => String::from("Unknown Response Content length"),
        };
        log::info!("Response content length: {content_lenght}",);

        // Todo: Optimize
        let stream = BufReader::new(resp);

        let deserialized: Vec<ScryfallCard> =
            serde_json::from_reader::<_, ScryfallMtgoCards>(stream)
                .unwrap()
                .0;

        log::info!("Got {} MTGO Scryfall cards", deserialized.len());

        Ok(Self {
            endpoint: url_endpoint,
            updated_at: date,
            cards: deserialized,
        })
    }

    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn cards(&self) -> &[ScryfallCard] {
        &self.cards
    }

    /// Take ownership of the [Vec] of [ScryfallCard], consuming [ScryfallBulkData] in the process
    pub fn take_cards(self) -> Vec<ScryfallCard> {
        self.cards
    }
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;
    use testresult::TestResult;

    use crate::util::init_debug_logging;

    use super::*;

    #[ignore = "Will download A LOT of data from the Scryfall API"]
    #[test]
    fn test_get_scryfall_bulk_data() -> TestResult {
        init_debug_logging(3);
        let date = Utc.with_ymd_and_hms(2024, 5, 19, 9, 5, 48).unwrap();
        let bulk_data = ScryfallBulkData::get(date)?;

        eprintln!("{bulk_data:?}");

        assert!(!bulk_data.cards().is_empty());

        let cards = bulk_data.take_cards();
        for c in &cards[0..=10] {
            dbg!(c);
        }

        Ok(())
    }
}
