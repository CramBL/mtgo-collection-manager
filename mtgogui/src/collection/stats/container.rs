use mtgoupdater::{MtgoCard, Rarity};

use super::util::{MultiValueStat, UniqueTotal};

/// Container for collection stats
#[derive(Debug, Clone)]
pub struct CollectionStats {
    total_cards: UniqueTotal,
    total_value: Option<MultiValueStat>,
    most_expensive_item: Option<MultiValueStat>,
    cards_under_a_tenth_tix: UniqueTotal,
    cards_over_5_tix: UniqueTotal,
    rarity_distribution: Option<MultiValueStat>,
}

impl CollectionStats {
    /// Create a new empty [CollectionStats] container
    pub fn new() -> Self {
        Self {
            total_value: None,
            total_cards: UniqueTotal::default(),
            most_expensive_item: None,
            cards_under_a_tenth_tix: UniqueTotal::default(),
            cards_over_5_tix: UniqueTotal::default(),
            rarity_distribution: None,
        }
    }

    /// Create a new [CollectionStats] from a list of cards
    ///
    /// # Arguments
    ///
    /// * `cards` - A borrowed slice of cards to create stats from
    ///
    /// # Returns
    ///
    /// A new [CollectionStats] container
    pub fn from_cards(cards: &[MtgoCard]) -> Self {
        let mut stats = Self::new();

        stats.calc_total_cards(cards);
        stats.calc_most_expensive_item(cards);
        stats.calc_total_value(cards);
        stats.cards_under_a_tenth_tix = Self::calc_cards_under_tix(0.1, cards);
        stats.cards_over_5_tix = Self::calc_cards_over_tix(5.0, cards);
        stats.rarity_distribution = Some(Self::calc_rarity_distribution(cards));
        stats
    }

    fn calc_total_cards(&mut self, cards: &[MtgoCard]) {
        let quantity_count = cards.iter().fold(0, |acc, card| acc + card.quantity);
        self.set_total_cards(cards.len(), quantity_count as usize);
    }

    fn calc_most_expensive_item(&mut self, cards: &[MtgoCard]) {
        let gb_most_expensive = cards
            .iter()
            .max_by(|a, b| a.goatbots_price.partial_cmp(&b.goatbots_price).unwrap())
            .unwrap_or_else(|| panic!("No cards in collection!"));
        let scryfall_most_expensive = cards
            .iter()
            .max_by(|a, b| {
                a.scryfall_price
                    .partial_cmp(&b.scryfall_price)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .unwrap_or_else(|| panic!("No cards in collection!"));

        let descriptions: Vec<String> = if gb_most_expensive.goatbots_price
            > scryfall_most_expensive.scryfall_price.unwrap_or(0.)
        {
            vec![
                gb_most_expensive.name.to_string(),
                format!(
                    "@C2@.{price} tix @Goatbots",
                    price = gb_most_expensive.goatbots_price
                ),
                format!(
                    "@C3@.{price} tix @Cardhoarder",
                    price = scryfall_most_expensive.scryfall_price.unwrap_or_default()
                ),
            ]
        } else {
            vec![
                scryfall_most_expensive.name.to_string(),
                format!(
                    "@C3@.{price} tix @Goatbots",
                    price = gb_most_expensive.goatbots_price
                ),
                format!(
                    "@C2@.{price} tix @Cardhoarder",
                    price = scryfall_most_expensive.scryfall_price.unwrap_or_default()
                ),
            ]
        };

        self.most_expensive_item = Some(MultiValueStat::new(
            "Most expensive item".to_string(),
            descriptions,
        ));
    }

    fn calc_total_value(&mut self, cards: &[MtgoCard]) {
        let total_gb_value = cards
            .iter()
            .map(|card| card.goatbots_price as f64 * card.quantity as f64)
            .sum::<f64>();
        let total_scryfall_value = cards.iter().fold(0., |acc, card| {
            acc + card
                .scryfall_price
                .map_or(0., |price| price as f64 * card.quantity as f64)
        });

        self.total_value = Some(MultiValueStat::new(
            "Total value".to_string(),
            if total_gb_value > total_scryfall_value {
                vec![
                    format!("@C2@.{:.2} tix @Goatbots", total_gb_value),
                    format!("@C3@.{:.2} tix @Cardhoarder", total_scryfall_value),
                ]
            } else {
                vec![
                    format!("@C3@.{:.2} tix @Goatbots", total_gb_value),
                    format!("@C2@.{:.2} tix @Cardhoarder", total_scryfall_value),
                ]
            },
        ));
    }

    fn calc_cards_under_tix(price: f32, cards: &[MtgoCard]) -> UniqueTotal {
        let (unique_count, quantity_count) = cards.iter().fold((0, 0), |acc, card| {
            if card.goatbots_price < price {
                (acc.0 + 1, acc.1 + card.quantity)
            } else {
                acc
            }
        });

        UniqueTotal::new(unique_count, quantity_count as usize)
    }

    fn calc_cards_over_tix(price: f32, cards: &[MtgoCard]) -> UniqueTotal {
        let (unique_count, quantity_count) = cards.iter().fold((0, 0), |acc, card| {
            if card.goatbots_price > price {
                (acc.0 + 1, acc.1 + card.quantity)
            } else {
                acc
            }
        });
        UniqueTotal::new(unique_count, quantity_count as usize)
    }

    fn calc_rarity_distribution(cards: &[MtgoCard]) -> MultiValueStat {
        let mut common = 0;
        let mut uncommon = 0;
        let mut rare = 0;
        let mut mythic = 0;

        cards.iter().for_each(|card| match card.rarity {
            Rarity::Common => common += 1,
            Rarity::Uncommon => uncommon += 1,
            Rarity::Rare => rare += 1,
            Rarity::Mythic => mythic += 1,
            _ => (),
        });

        let total = common + uncommon + rare + mythic;
        let percent_common = common as f32 / total as f32 * 100.;
        let percent_uncommon = uncommon as f32 / total as f32 * 100.;
        let percent_rare = rare as f32 / total as f32 * 100.;
        let percent_mythic = mythic as f32 / total as f32 * 100.;

        MultiValueStat::new(
            "Rarity distribution".to_string(),
            vec![
                format!("@F13@.{total} unique cards"),
                format!(
                    "@F13@C90@.Mythic   {mythic:>5} ({percent:>5}%)",
                    // Two steps formatting to get the correct number of decimals AND the correct alignment
                    // Does not work with format_args!() macro unfortunately
                    percent = format!("{percent_mythic:.2}")
                ),
                format!(
                    "@F13@C93@.Rare     {rare:>5} ({percent:>5}%)",
                    percent = format!("{percent_rare:.2}")
                ),
                format!(
                    "@F13@C26@.Uncommon {uncommon:>5} ({percent:>5}%)",
                    percent = format!("{percent_uncommon:.2}")
                ),
                format!(
                    "@F13@C15@.Common   {common:>5} ({percent:>5}%)",
                    percent = format!("{percent_common:.2}")
                ),
            ],
        )
    }

    pub fn set_total_cards(&mut self, total_unique_cards: usize, total_card_quantity: usize) {
        self.total_cards = UniqueTotal::new(total_unique_cards, total_card_quantity);
    }

    pub fn set_cards_under_a_tenth_tix(
        &mut self,
        cards_under_tenth_tix_unique: usize,
        cards_under_tenth_tix_quantity: usize,
    ) {
        self.cards_under_a_tenth_tix =
            UniqueTotal::new(cards_under_tenth_tix_unique, cards_under_tenth_tix_quantity);
    }

    pub fn set_cards_over_5_tix(
        &mut self,
        cards_over_5_tix_unique: usize,
        cards_over_5_tix_quantity: usize,
    ) {
        self.cards_over_5_tix =
            UniqueTotal::new(cards_over_5_tix_unique, cards_over_5_tix_quantity);
    }

    pub fn total_cards(&self) -> UniqueTotal {
        self.total_cards
    }

    pub fn cards_under_a_tenth_tix(&self) -> UniqueTotal {
        self.cards_under_a_tenth_tix
    }

    pub fn cards_over_5_tix(&self) -> UniqueTotal {
        self.cards_over_5_tix
    }

    pub fn take_total_value(&mut self) -> Option<MultiValueStat> {
        self.total_value.take()
    }

    pub fn take_rarity_distribution(&mut self) -> Option<MultiValueStat> {
        self.rarity_distribution.take()
    }

    pub fn take_most_expensive_item(&mut self) -> Option<MultiValueStat> {
        self.most_expensive_item.take()
    }
}

impl Default for CollectionStats {
    fn default() -> Self {
        Self::new()
    }
}
