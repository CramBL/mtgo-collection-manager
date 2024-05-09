use reqwest::Url;
use std::io::{Cursor, Read};

pub mod state_log;

const GOATBOTS_PRICE_HISTORY_URL: &str = "https://www.goatbots.com/download/price-history.zip";

pub fn get_goatbots_price_history() -> String {
    let bytes = reqwest::blocking::get(GOATBOTS_PRICE_HISTORY_URL)
        .unwrap()
        .bytes()
        .unwrap();

    eprintln!("Got {} bytes", bytes.len());
    let cursor = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor).unwrap();

    let mut file = archive.by_index(0).unwrap();
    let mut contents = String::with_capacity(512 * 1024);
    file.read_to_string(&mut contents).unwrap();
    contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goatbots_price_history_url() {
        assert!(Url::parse(GOATBOTS_PRICE_HISTORY_URL).is_ok());
    }

    #[test]
    fn test_get_goatbots_price_history() {
        let res = get_goatbots_price_history();
        eprintln!("{res}");
    }
}
