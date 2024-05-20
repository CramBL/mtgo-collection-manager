use std::io::{self, Read};

use reqwest::blocking;

const GOATBOTS_PRICE_HISTORY_URL: &str = "https://www.goatbots.com/download/price-history.zip";
const GOATBOTS_CARD_DEFINITIONS_URL: &str =
    "https://www.goatbots.com/download/card-definitions.zip";

fn fetch_and_extract_content(url: &str) -> String {
    let resp = blocking::get(url).unwrap();
    let bytes = resp.bytes().unwrap();

    log::info!("Got {} bytes", bytes.len());
    let cursor = io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();

    let mut file = archive.by_index(0).unwrap();
    let mut contents = String::with_capacity(512 * 1024);
    file.read_to_string(&mut contents).unwrap();
    contents
}

/// Get Goatbots Price History
///
/// TODO: Error handling, this should return a result
pub fn get_goatbots_price_history() -> String {
    fetch_and_extract_content(GOATBOTS_PRICE_HISTORY_URL)
}

/// Get Goatbots Card Definitions
///
/// TODO: Error handling, this should return a result
pub fn get_goatbots_card_definitions() -> String {
    fetch_and_extract_content(GOATBOTS_CARD_DEFINITIONS_URL)
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Url;

    #[test]
    fn test_urls() {
        assert!(Url::parse(GOATBOTS_PRICE_HISTORY_URL).is_ok());
        assert!(Url::parse(GOATBOTS_CARD_DEFINITIONS_URL).is_ok());
    }

    #[ignore = "Will download data from the goatbots website"]
    #[test]
    fn test_get_goatbots_price_history() {
        let res = get_goatbots_price_history();
        eprintln!("{res}");
        assert_ne!(res.len(), 0);
    }

    #[ignore = "Will download data from the goatbots website"]
    #[test]
    fn test_get_goatbots_card_definitions() {
        let res = get_goatbots_card_definitions();
        eprintln!("{res}");
        assert_ne!(res.len(), 0);
    }
}
