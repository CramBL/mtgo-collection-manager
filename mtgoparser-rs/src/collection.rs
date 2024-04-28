use crate::{
    goatbots::card_definitions::GoatBotsCard,
    mtgo_card::{MtgoCard, Rarity},
    scryfall::default_cards::ScryfallCard,
    xml::XmlCard,
};
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, num::ParseIntError};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Collection {
    cards: Vec<MtgoCard>,
    total_quantity: Option<u32>,
}

impl Collection {
    pub fn from_xml_cards(cards: Vec<XmlCard>) -> Result<Self, ParseIntError> {
        let mut mtgo_cards = Vec::<MtgoCard>::with_capacity(cards.len());

        for card in cards.into_iter() {
            mtgo_cards.push(MtgoCard::from_xml_card(card)?);
        }

        Ok(Self {
            cards: mtgo_cards,
            total_quantity: None,
        })
    }

    pub fn extract_goatbots_info(
        &mut self,
        mut card_defs: HashMap<String, GoatBotsCard>,
        price_hist: HashMap<String, f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for card in self.cards.iter_mut() {
            if card.id == 1 {
                card.goatbots_price = 1.0; // Event tickets have value 1 per definition
                continue;
            }

            if let Some(cd) = card_defs.remove(&card.id.to_string()) {
                card.set = cd.cardset.into_boxed_str();
                card.rarity = Rarity::from(cd.rarity.as_ref());
                card.foil = cd.foil == 1;
            } else {
                eprintln!("Card definition key not found: ID={}", card.id);
            }

            if let Some(price) = price_hist.get(&card.id.to_string()) {
                card.goatbots_price = *price;
            } else {
                eprintln!("Price history key not found: ID={}", card.id);
            }
        }

        Ok(())
    }

    pub fn extract_scryfall_info(
        &mut self,
        mut scryfall_cards: Vec<ScryfallCard>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        scryfall_cards.sort_unstable_by_key(|k| k.mtgo_id);

        // Iterate over all the mtgo cards and the scryfall card info
        // If matching on the ID, assign the scryfall price
        // If mtgo card id is higher then scryfall id -> check the next scryfall card
        // If mtgo card id is lower than scryfall id -> check next mtgo card id
        // Loop until one of the collections is exhausted.
        let mut scry_idx = 0;
        for card in self.cards.iter_mut() {
            // Skip if it is foil as scryfall API doesn't have foil prices
            if card.foil {
                continue;
            }
            if card.id == 1 {
                // Event ticket
                card.scryfall_price = Some(1.0);
                continue;
            }

            while let Some(sc) = scryfall_cards.get_mut(scry_idx) {
                if sc.mtgo_id == card.id {
                    if let Some(tix_price) = sc.prices.tix.take() {
                        card.scryfall_price = Some(tix_price.parse()?);
                    }
                }
                scry_idx += 1;
            }
        }
        Ok(())
    }

    pub fn unique_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn total_cards(&mut self) -> u32 {
        if let Some(quantity) = self.total_quantity {
            quantity
        } else {
            let quantity = self.cards.iter().map(|c| c.quantity).sum();
            self.total_quantity = Some(quantity);
            quantity
        }
    }

    pub fn take_cards(&mut self) -> Vec<MtgoCard> {
        self.cards.drain(..).collect()
    }
}
