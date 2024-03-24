use std::path::Path;

pub mod collection;
pub mod goatbots;
pub mod mtgo_card;
pub mod scryfall;
pub mod xml;

pub fn update(full_trade_list: &Path) {}

#[cfg(test)]
mod tests {
    use super::*;
}
