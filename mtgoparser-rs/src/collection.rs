use std::num::ParseIntError;

use crate::{mtgo_card::MtgoCard, xml::XmlCard};

pub struct Collection {
    cards: Vec<MtgoCard>,
}

impl Collection {
    pub fn from_xml_cards(cards: Vec<XmlCard>) -> Result<Self, ParseIntError> {
        let mut mtgo_cards = Vec::<MtgoCard>::with_capacity(cards.len());

        for card in cards.into_iter() {
            mtgo_cards.push(MtgoCard::from_xml_card(card)?);
        }

        Ok(Self { cards: mtgo_cards })
    }
}
