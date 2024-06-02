use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct PriceHistoryTracker {
    pub quantity: Option<u32>,
    pub goatbots_price: Option<f32>,
    pub scryfall_price: Option<f32>,
}

impl PriceHistoryTracker {
    pub fn new(
        quantity: Option<u32>,
        goatbots_price: Option<f32>,
        scryfall_price: Option<f32>,
    ) -> Self {
        Self {
            quantity,
            goatbots_price,
            scryfall_price,
        }
    }
}
