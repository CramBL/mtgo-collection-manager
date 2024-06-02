use super::{MtgoCard, Rarity};
use serde::{Deserialize, Serialize};

pub use price_history_tracker::PriceHistoryTracker;

pub mod price_history_tracker;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct CardHistory {
    pub id: u32,
    pub quantity: String,
    pub name: String,
    pub set: String,
    pub rarity: Rarity,
    pub foil: bool,
    pub price_history: Vec<PriceHistoryTracker>,
}

impl CardHistory {
    pub fn new(
        id: u32,
        quantity: String,
        name: String,
        set: String,
        rarity: Rarity,
        foil: bool,
        price_history: Vec<PriceHistoryTracker>,
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
        let price_history_tracker = PriceHistoryTracker {
            quantity: Some(card.quantity),
            goatbots_price: Some(card.goatbots_price),
            scryfall_price: card.scryfall_price,
        };
        let price_history = vec![price_history_tracker];
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

        for PriceHistoryTracker {
            quantity,
            goatbots_price,
            scryfall_price,
        } in &self.price_history
        {
            s.push(',');
            match quantity {
                Some(q) => {
                    s.push('[');
                    s.push_str(&q.to_string());
                    s.push(']');
                }
                None => (),
            }
            match goatbots_price {
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

    pub fn from_csv_row(row: String) -> Result<Self, Box<dyn std::error::Error>> {
        let mut iter: std::str::Split<char> = row.split(',');
        let id = csv_row_helper::col0_id(&mut iter)?;
        let quantity = csv_row_helper::col1_quantity(&mut iter)?.to_string();
        let name = csv_row_helper::col2_name(&mut iter);
        let set = csv_row_helper::col3_set(&mut iter);
        let rarity = csv_row_helper::col4_rarity(&mut iter);
        let foil = csv_row_helper::col5_foil(&mut iter)?;
        let price_history = csv_row_helper::col6_to_col_n(&mut iter);
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

// Helpers for making it more readable to parse from csv frow
mod csv_row_helper {
    use std::{
        num::ParseIntError,
        str::{ParseBoolError, Split},
    };

    use crate::mtgo_card::Rarity;

    use super::PriceHistoryTracker;
    // Advances the iterator and parses the next element as u32
    fn next_parse_to_int(iter: &mut Split<char>) -> Result<u32, ParseIntError> {
        iter.next().unwrap().parse()
    }

    // Advances the iterator and returns the next element as an owned string
    fn next_to_string(iter: &mut Split<char>) -> String {
        iter.next().unwrap().to_string()
    }

    // Advances the iterator and parses the next element as bool
    fn next_parse_to_bool(iter: &mut Split<char>) -> Result<bool, ParseBoolError> {
        iter.next().unwrap().parse()
    }

    // First column contains id
    pub fn col0_id(iter: &mut Split<char>) -> Result<u32, ParseIntError> {
        next_parse_to_int(iter)
    }

    pub fn col1_quantity(iter: &mut Split<char>) -> Result<u32, ParseIntError> {
        next_parse_to_int(iter)
    }

    // The name string is quoted (because MTG names can contain all sorts of characters) so we remove the extra quotes after parsing
    pub fn col2_name(iter: &mut Split<char>) -> String {
        let first_element = iter.next().unwrap();
        // If the first element ends with '"' then it contained the entire name and we return
        if first_element.ends_with('"') {
            return first_element[1..first_element.len() - 1].to_string();
        }
        // If the first element doesn't end with '"' that means it contains
        // commas (,) and was split over several element in the CSV row, so we iterate
        // until we find the element that ends with '"' and add all ther intermediate strings
        // to the following variable:
        let mut name = first_element[1..].to_string();

        let mut found_end: bool = false;
        while let Some(str_element) = iter.next() {
            // Every additional element has to be prefixed with a comma since it was
            //  stripped by the split(',')-call
            name.push(',');
            if str_element.ends_with('"') {
                found_end = true;
                name.push_str(&str_element[0..str_element.len() - 1]);
                break;
            } else {
                // Another substring so we add it and continue
                name.push_str(str_element);
            }
        }
        if !found_end {
            panic!("Reached end while parsing card history card name without encountering a closing quote (\")")
        }
        name
    }

    pub fn col3_set(iter: &mut Split<char>) -> String {
        next_to_string(iter)
    }

    pub fn col4_rarity(iter: &mut Split<char>) -> Rarity {
        Rarity::from(iter.next().unwrap())
    }

    pub fn col5_foil(iter: &mut Split<char>) -> Result<bool, ParseBoolError> {
        next_parse_to_bool(iter)
    }

    // Parse a price element from the `PriceHistoryTracker` part of a CSV row
    fn parse_price_element(price_iter: &mut Split<char>) -> Option<f32> {
        match price_iter.next().unwrap() {
            "-" => None,
            p => Some(p.parse().unwrap()),
        }
    }

    fn parse_quantity_and_goatbots_element(
        price_iter: &mut Split<char>,
    ) -> (Option<u32>, Option<f32>) {
        let quant_and_gb_element = price_iter.next().unwrap();
        let element_has_quantity = quant_and_gb_element.get(0..1).unwrap() == "[";

        let quantity_end_pos: Option<usize> = quant_and_gb_element.find(']');

        let quantity: Option<u32> = if element_has_quantity {
            let quantity_str = quant_and_gb_element
                .get(1..quantity_end_pos.unwrap())
                .unwrap();
            Some(quantity_str.parse().unwrap())
        } else {
            None
        };

        let goatbots_price_str = if element_has_quantity {
            quant_and_gb_element
                .get(quantity_end_pos.unwrap() + 1..)
                .unwrap()
        } else {
            quant_and_gb_element
        };
        let goatbots_price: Option<f32> = Some(goatbots_price_str.parse().unwrap());

        (quantity, goatbots_price)
    }

    pub fn col6_to_col_n(iter: &mut Split<char>) -> Vec<PriceHistoryTracker> {
        let mut price_history = Vec::new();
        for price in iter {
            let mut price_iter = price.split(';');
            let (quantity, goatbots_price) = parse_quantity_and_goatbots_element(&mut price_iter);
            let scryfall_price = parse_price_element(&mut price_iter);
            price_history.push(PriceHistoryTracker {
                quantity,
                goatbots_price,
                scryfall_price,
            });
        }
        price_history
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    use super::*;

    const EXAMPLE_BLACK_LOTUS_CSV_ROW: &str = r#"1,3,"Black Lotus",1E,Rare,false,[3]1.23;3.45"#;
    // Name and set are heap allocated so we cannot simply make a const instance
    fn example_black_lotus_mtgo_card() -> MtgoCard {
        MtgoCard {
            id: 1,
            quantity: 3,
            name: "Black Lotus".into(),
            set: "1E".into(),
            rarity: Rarity::Rare,
            foil: false,
            goatbots_price: 1.23,
            scryfall_price: Some(3.45),
        }
    }

    const EXAMPLE_ARLINN_THE_PACKS_HOPE_CSV_ROW: &str = r#"94060,40,"Arlinn, the Pack's Hope",MID,Mythic,false,[1004]2.1;-,[900]3;3.04,[1]7.03;10.9,9.93;10,8;8,4.57;4.1,2.57;-,[40]2.57;2"#;
    fn example_arlinn_the_packs_hope_card_history() -> CardHistory {
        CardHistory {
            id: 94060,
            quantity: "40".into(),
            name: "Arlinn, the Pack's Hope".into(),
            set: "MID".into(),
            rarity: Rarity::Mythic,
            foil: false,
            // A history with selling a big quantity as price goes up, eventually having a quantity of 0
            // then when price is low again, buying to a quantity of 40
            price_history: vec![
                PriceHistoryTracker {
                    quantity: Some(1004),
                    goatbots_price: Some(2.1),
                    scryfall_price: None,
                },
                PriceHistoryTracker {
                    quantity: Some(900),
                    goatbots_price: Some(3.),
                    scryfall_price: Some(3.04),
                },
                PriceHistoryTracker {
                    quantity: Some(1),
                    goatbots_price: Some(7.03),
                    scryfall_price: Some(10.9),
                },
                PriceHistoryTracker {
                    quantity: None,
                    goatbots_price: Some(9.93),
                    scryfall_price: Some(10.),
                },
                PriceHistoryTracker {
                    quantity: None,
                    goatbots_price: Some(8.),
                    scryfall_price: Some(8.),
                },
                PriceHistoryTracker {
                    quantity: None,
                    goatbots_price: Some(4.57),
                    scryfall_price: Some(4.1),
                },
                PriceHistoryTracker {
                    quantity: None,
                    goatbots_price: Some(2.57),
                    scryfall_price: None,
                },
                PriceHistoryTracker {
                    quantity: Some(40),
                    goatbots_price: Some(2.57),
                    scryfall_price: Some(2.),
                },
            ],
        }
    }

    #[test]
    pub fn test_to_csv_row() -> TestResult {
        let mtgo_card = example_black_lotus_mtgo_card();
        let card_history = CardHistory::from_mtgo_card(mtgo_card.clone());
        assert_eq!(card_history.id, mtgo_card.id);

        let csv_row = card_history.to_csv_row();
        assert_eq!(csv_row, EXAMPLE_BLACK_LOTUS_CSV_ROW);

        Ok(())
    }

    #[test]
    pub fn test_from_csv_row() -> TestResult {
        let card_history =
            CardHistory::from_csv_row(EXAMPLE_BLACK_LOTUS_CSV_ROW.to_owned()).unwrap();

        assert_eq!(
            card_history,
            CardHistory::from_mtgo_card(example_black_lotus_mtgo_card())
        );

        Ok(())
    }

    #[test]
    pub fn test_to_csv_row_with_longer_history() -> TestResult {
        let card_history = example_arlinn_the_packs_hope_card_history();
        let csv_row = card_history.to_csv_row();
        assert_eq!(csv_row, EXAMPLE_ARLINN_THE_PACKS_HOPE_CSV_ROW);
        Ok(())
    }

    #[test]
    pub fn test_from_csv_row_with_longer_history() -> TestResult {
        let card_history =
            CardHistory::from_csv_row(EXAMPLE_ARLINN_THE_PACKS_HOPE_CSV_ROW.to_owned()).unwrap();

        assert_eq!(card_history, example_arlinn_the_packs_hope_card_history());

        Ok(())
    }
}
