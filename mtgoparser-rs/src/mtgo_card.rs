use std::num::ParseIntError;

pub mod card_history;
pub mod collection_history;

use serde_derive::{Deserialize, Serialize};

use crate::xml::XmlCard;

/// This is the struct that represents a card in the MTGO collection.
///
/// It is not the same as a paper card which can have additional fields such as released_at and prices in USD and EUR.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct MtgoCard {
    pub id: u32,
    pub quantity: u32,
    pub name: Box<str>,
    pub set: Box<str>,
    pub rarity: Rarity,
    pub foil: bool,
    pub goatbots_price: f32,
    pub scryfall_price: Option<f32>,
}

impl MtgoCard {
    /// Create an instance of [MtgoCard] from an [XmlCard]. An [XmlCard] only has 3 values, so the remaining
    /// values of the [MtgoCard] is set to default values.
    pub fn from_xml_card(card: XmlCard) -> Result<Self, ParseIntError> {
        Ok(Self {
            id: card.cat_id.parse()?,
            quantity: card.quantity.parse()?,
            name: card.name.into_boxed_str(),
            ..Default::default()
        })
    }
}

/// Represents the rarity of an MTGO item (e.g. card, booster, event ticket)
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, PartialOrd, Ord, Eq)]
pub enum Rarity {
    #[default]
    #[serde(alias = "C")]
    Common,
    #[serde(alias = "U")]
    Uncommon,
    #[serde(alias = "R")]
    Rare,
    #[serde(alias = "M")]
    Mythic,
    #[serde(alias = "B")]
    Booster,
    #[serde(other)]
    None,
}

impl ToString for Rarity {
    fn to_string(&self) -> String {
        match self {
            Rarity::Common => "Common".into(),
            Rarity::Uncommon => "Uncommon".into(),
            Rarity::Rare => "Rare".into(),
            Rarity::Mythic => "Mythic".into(),
            Rarity::Booster => "Booster".into(),
            Rarity::None => "None".into(),
        }
    }
}

impl From<&str> for Rarity {
    fn from(s: &str) -> Self {
        match s {
            // Single letter matches first for speed
            "C" => Rarity::Common,
            "U" => Rarity::Uncommon,
            "R" => Rarity::Rare,
            "M" => Rarity::Mythic,
            "B" => Rarity::Booster,
            "Uncommon" => Rarity::Uncommon,
            "Rare" => Rarity::Rare,
            "Mythic" => Rarity::Mythic,
            "Booster" => Rarity::Booster,
            "Common" => Rarity::Common,
            _ => Rarity::None, // e.g. Event tickets
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_serialize() {
        let c = MtgoCard::default();

        let serialized = serde_json::to_string_pretty(&c).unwrap();
        println!("serialized = {serialized}");

        let deserialized: MtgoCard = serde_json::from_str(&serialized).unwrap();
        assert_eq!(c, deserialized);
    }

    #[test]
    fn test_deserialize_json_string_vector() {
        let json_vec_str = r#"[
         {
            "id": 1,
            "quantity": 391,
            "name": "Event Ticket",
            "set": "",
            "rarity": "",
            "foil": false,
            "goatbots_price": 0
         },
         {
            "id": 235,
            "quantity": 1,
            "name": "Swamp",
            "set": "PRM",
            "rarity": "Common",
            "foil": false,
            "goatbots_price": 0.002,
            "scryfall_price": 0.05
         }
      ]"#;

        let deserialized: Vec<MtgoCard> = serde_json::from_str(json_vec_str).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(deserialized[0].id, 1);
        assert_eq!(deserialized[0].quantity, 391);
        assert_eq!(deserialized[0].name, "Event Ticket".into());
        assert_eq!(deserialized[0].set, "".into());
        assert_eq!(deserialized[0].rarity, Rarity::None);
        assert_eq!(deserialized[0].goatbots_price, 0.0);
        assert_eq!(deserialized[0].scryfall_price, None);

        assert_eq!(deserialized[1].id, 235);
        assert_eq!(deserialized[1].quantity, 1);
        assert_eq!(deserialized[1].name, "Swamp".into());
        assert_eq!(deserialized[1].set, "PRM".into());
        assert_eq!(deserialized[1].rarity, Rarity::Common);
        assert_eq!(deserialized[1].goatbots_price, 0.002);
        assert_eq!(deserialized[1].scryfall_price, Some(0.05));
    }
}
