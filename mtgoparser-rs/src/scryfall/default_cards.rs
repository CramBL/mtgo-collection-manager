use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Prices {
    usd: Option<String>,
    usd_foil: Option<String>,
    eur: Option<String>,
    eur_foil: Option<String>,
    tix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct ScryfallCard {
    pub mtgo_id: u32,
    pub name: String,
    pub released_at: String,
    pub rarity: String,
    pub prices: Prices,
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    #[test]
    pub fn scryfall_parse() -> TestResult {
        let scryfall_json_str =
            std::fs::read_to_string("../test/test-data/scryfall/default-cards-small-5cards.json")?;

        let scryfall_cards: Vec<ScryfallCard> = serde_json::from_str(&scryfall_json_str)?;

        assert_eq!(
            scryfall_cards[0],
            ScryfallCard {
                mtgo_id: 25527,
                name: "Fury Sliver".into(),
                released_at: "2006-10-06".into(),
                rarity: "uncommon".into(),
                prices: Prices {
                    usd: Some("0.47".into()),
                    usd_foil: Some("4.48".into()),
                    eur: Some("0.20".into()),
                    eur_foil: Some("0.50".into()),
                    tix: Some("0.03".into())
                }
            }
        );

        Ok(())
    }
}
