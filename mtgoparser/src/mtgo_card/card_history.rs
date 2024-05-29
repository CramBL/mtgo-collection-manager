use super::{MtgoCard, Rarity};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CardHistory {
    pub id: u32,
    pub quantity: String,
    pub name: String,
    pub set: String,
    pub rarity: Rarity,
    pub foil: bool,
    pub price_history: Vec<(Option<u32>, Option<f32>, Option<f32>)>,
}

impl CardHistory {
    pub fn new(
        id: u32,
        quantity: String,
        name: String,
        set: String,
        rarity: Rarity,
        foil: bool,
        price_history: Vec<(Option<u32>, Option<f32>, Option<f32>)>,
    ) -> Self {
        Self {
            id,
            quantity,
            name,
            set,
            rarity,
            foil,
            price_history,
        }
    }

    pub fn from_mtgo_card(card: MtgoCard) -> Self {
        let price_history = vec![(
            Some(card.quantity),
            Some(card.goatbots_price),
            card.scryfall_price,
        )];
        Self {
            id: card.id,
            quantity: card.quantity.to_string(),
            name: card.name.to_string(),
            set: card.set.to_string(),
            rarity: card.rarity,
            foil: card.foil,
            price_history,
        }
    }

    pub fn to_csv_row(&self) -> String {
        let mut s = String::with_capacity(512);
        s.push_str(&self.id.to_string());
        s.push(',');
        s.push_str(&self.quantity);
        s.push(',');
        s.push('"');
        s.push_str(&self.name);
        s.push('"');
        s.push(',');
        s.push_str(&self.set);
        s.push(',');
        s.push_str(&self.rarity.to_string());
        s.push(',');
        s.push_str(&self.foil.to_string());

        for (quantity, gb_price, scryfall_price) in &self.price_history {
            s.push(',');
            match quantity {
                Some(q) => s.push_str(&format!("[{q}]")),
                None => s.push_str(""),
            }
            match gb_price {
                Some(p) => s.push_str(p.to_string().as_str()),
                None => s.push('-'),
            }
            s.push(';');
            match scryfall_price {
                Some(p) => s.push_str(p.to_string().as_str()),
                None => s.push('-'),
            }
        }
        s
    }

    pub fn csv_row_to_card_history(row: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut iter = row.split(',');
        let id = iter.next().unwrap().parse()?;
        let quantity = iter.next().unwrap().to_string();
        let name = iter.next().unwrap().to_string();
        let set = iter.next().unwrap().to_string();
        let rarity = Rarity::from(iter.next().unwrap());
        let foil = iter.next().unwrap().parse()?;
        let mut price_history = Vec::new();
        for price in iter {
            let mut price_iter = price.split(';');
            let quantity = match price_iter.next().unwrap() {
                "" => None,
                q => Some(q.parse().unwrap()),
            };
            let gb_price = match price_iter.next().unwrap() {
                "-" => None,
                p => Some(p.parse().unwrap()),
            };
            let scryfall_price = match price_iter.next().unwrap() {
                "-" => None,
                p => Some(p.parse().unwrap()),
            };
            price_history.push((quantity, gb_price, scryfall_price));
        }
        Ok(Self {
            id,
            quantity,
            name,
            set,
            rarity,
            foil,
            price_history,
        })
    }
}
