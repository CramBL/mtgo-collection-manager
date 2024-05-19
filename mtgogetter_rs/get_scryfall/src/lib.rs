

use serde::{Deserialize, Serialize};


pub mod bulk_cards;
pub mod bulk_info;
pub mod util;

////////////////////////////////
#[derive(Deserialize, Serialize)]
struct ScryfallSet {
    name: String,
    released_at: String,
    mtgo_code: String,
}

#[derive(Deserialize, Serialize)]
struct ScryfallCard {
    mtgo_id: i32,
    name: String,
    released_at: String,
    rarity: String,
    prices: Prices,
}

#[derive(Deserialize, Serialize)]
struct Prices {
    usd: Option<String>,
    usd_foil: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
    tix: Option<String>,
}

#[cfg(test)]
mod tests {
    use testresult::TestResult;

    use super::*;

    #[ignore = "Will download data from the Scryfall API"]
    #[test]
    fn test_get_scryfall_bulk_info() -> TestResult {
        Ok(())
    }
}
