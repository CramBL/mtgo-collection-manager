use reqwest::Url;

pub mod state_log;


const GOATBOTS_PRICE_HISTORY_URL: &str = "https://www.goatbots.com/download/price-history.zip";

pub fn get_goatbots_price_history() {
    let bytes = reqwest::blocking::get(GOATBOTS_PRICE_HISTORY_URL)
        .unwrap()
        .bytes()
        .unwrap();

    eprintln!("Got {} bytes", bytes.len());
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
        get_goatbots_price_history();
    }
}
