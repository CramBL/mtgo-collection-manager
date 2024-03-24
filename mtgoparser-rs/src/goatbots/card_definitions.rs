use serde_derive::{Deserialize, Serialize};
use std::{collections::hash_map::HashMap, path::Path};

/// The relevant card information that can be extracted from a GoatBots JSON file.
/// e.g.
/// ```json
/// {
///     "47483": {
///        "name": "Gruul Charm",
///        "cardset": "GTC",
///        "rarity": "Uncommon",
///        "foil": 0
///     },
///    ...
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct GoatBotsCard {
    pub name: String,
    pub cardset: String,
    pub rarity: String,
    pub foil: u8,
}

impl GoatBotsCard {
    pub fn new(name: String, cardset: String, rarity: String, foil: u8) -> Self {
        Self {
            name,
            cardset,
            rarity,
            foil,
        }
    }
}

pub fn parse_card_def_json(
    path: &Path,
) -> Result<HashMap<String, GoatBotsCard>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(path)?;
    let cards: HashMap<String, GoatBotsCard> = serde_json::from_str(&json)?;
    Ok(cards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    #[test]
    fn test_new_goatbots_card() -> TestResult {
        let card = GoatBotsCard::new(
            "Island".to_string(),
            "M20".to_string(),
            "Common".to_string(),
            0,
        );
        assert_eq!(card.name, "Island");
        assert_eq!(card.cardset, "M20");
        assert_eq!(card.rarity, "Common");
        assert_eq!(card.foil, 0);
        Ok(())
    }

    #[test]
    fn test_parse_goatbots_json_5_cards() -> TestResult {
        let path = Path::new(r"../test/test-data/goatbots/card-defs-small-5cards.json");
        let cards = parse_card_def_json(path)?;
        assert_eq!(cards.len(), 5);
        assert_eq!(
            cards.get("47483").unwrap(),
            &GoatBotsCard {
                name: "Gruul Charm".to_string(),
                cardset: "GTC".to_string(),
                rarity: "Uncommon".to_string(),
                foil: 0
            }
        );
        assert_eq!(
            cards.get("348").unwrap(),
            &GoatBotsCard {
                name: "Black Lotus".to_string(),
                cardset: "1E".to_string(),
                rarity: "Rare".to_string(),
                foil: 1
            }
        );
        Ok(())
    }

    #[test]
    fn test_parse_goatbots_json_full() -> TestResult {
        let path = Path::new(r"../test/test-data/goatbots/card-definitions-2023-10-02-full.json");
        let cards = parse_card_def_json(path)?;
        assert_eq!(cards.len(), 76070);
        assert_eq!(
            cards.get("47483").unwrap(),
            &GoatBotsCard {
                name: "Gruul Charm".to_string(),
                cardset: "GTC".to_string(),
                rarity: "Uncommon".to_string(),
                foil: 0
            }
        );
        assert_eq!(
            cards.get("348").unwrap(),
            &GoatBotsCard {
                name: "Black Lotus".to_string(),
                cardset: "1E".to_string(),
                rarity: "Rare".to_string(),
                foil: 1
            }
        );
        Ok(())
    }
}
