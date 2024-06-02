use std::cmp::Ordering;

use super::{
    card_history::{CardHistory, PriceHistoryTracker},
    MtgoCard,
};

#[derive(Debug, Clone, PartialEq)]
pub struct CardHistoryAggregate {
    pub card_history: CardHistory,
    // The newest quantity is the last time the quantity changed
    // Used to determine if the history should include a quantity update
    pub newest_quantity: u32,
}

impl CardHistoryAggregate {
    pub fn new(card_history: CardHistory, newest_quantity: u32) -> Self {
        Self {
            card_history,
            newest_quantity,
        }
    }
}

pub struct CollectionHistory {
    pub timestamps: Vec<String>,
    pub card_histories: Vec<CardHistoryAggregate>,
}

impl CollectionHistory {
    pub fn from_card_history(timestamps: Vec<String>, card_history: Vec<CardHistory>) -> Self {
        debug_assert_eq!(
            timestamps.len(),
            // Choose last because first will always be tix
            card_history.last().unwrap().price_history.len(),
            "Expected timestamps and length of price_history to be the same"
        );
        let mut card_histories: Vec<CardHistoryAggregate> = vec![];
        for cardh in card_history.into_iter() {
            let mut newest_quantity = 0;
            for PriceHistoryTracker {
                quantity,
                goatbots_price: _,
                scryfall_price: _,
            } in cardh.price_history.iter().rev()
            {
                if let Some(q) = quantity {
                    newest_quantity = *q;
                    break;
                }
            }
            card_histories.push(CardHistoryAggregate::new(cardh, newest_quantity));
        }

        Self {
            timestamps,
            card_histories,
        }
    }

    pub fn add_new_card_data(&mut self, timestamp: String, cards: Vec<MtgoCard>) {
        let mut card_history: Vec<CardHistory> =
            cards.into_iter().map(CardHistory::from_mtgo_card).collect();
        card_history.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        debug_assert!(card_history.first().unwrap().id < card_history.last().unwrap().id);
        self.card_histories
            .sort_unstable_by(|a, b| a.card_history.id.cmp(&b.card_history.id));
        debug_assert!(
            self.card_histories.first().unwrap().card_history.id
                < self.card_histories.last().unwrap().card_history.id
        );
        // Make the card history aligned to the current collection history
        let mut new_cards: Vec<CardHistory> = vec![];
        let mut new_cards_iter = card_history.into_iter();
        let mut current_new_card = new_cards_iter.next();
        let mut iterate_new_card = false;

        for old_card in self.card_histories.iter_mut() {
            if iterate_new_card {
                iterate_new_card = false;
                current_new_card = new_cards_iter.next();
            }
            let old_card_id = old_card.card_history.id;
            if let Some(new_card) = current_new_card.as_mut() {
                let new_id = new_card.id;
                match old_card_id.cmp(&new_id) {
                    Ordering::Less => {
                        old_card.newest_quantity = 0;
                        old_card
                            .card_history
                            .price_history
                            .push(PriceHistoryTracker::new(Some(0), None, None));
                    }
                    Ordering::Equal => {
                        debug_assert_eq!(new_card.price_history.len(), 1);
                        let mut price_history = new_card.price_history.pop().unwrap();
                        // If the quantity didn't change, we don't add it to the data.
                        let new_card_quantity = price_history.quantity.unwrap();
                        if new_card_quantity == old_card.newest_quantity {
                            price_history.quantity = None;
                        } else {
                            old_card.newest_quantity = new_card_quantity;
                        }
                        old_card.card_history.price_history.push(price_history);

                        iterate_new_card = true;
                    }
                    Ordering::Greater => {
                        new_cards.push(new_card.clone());
                        iterate_new_card = true;
                        loop {
                            if iterate_new_card {
                                current_new_card = new_cards_iter.next();
                            }
                            if let Some(new_card) = current_new_card.as_mut() {
                                let new_id = new_card.id;
                                match old_card_id.cmp(&new_id) {
                                    Ordering::Less => {
                                        old_card.newest_quantity = 0;
                                        old_card
                                            .card_history
                                            .price_history
                                            .push(PriceHistoryTracker::new(Some(0), None, None));
                                        break;
                                    }
                                    Ordering::Equal => {
                                        debug_assert_eq!(new_card.price_history.len(), 1);
                                        let mut price_history =
                                            new_card.price_history.pop().unwrap();
                                        // If the quantity didn't change, we don't add it to the data.
                                        let new_card_quantity = price_history.quantity.unwrap();
                                        if new_card_quantity == old_card.newest_quantity {
                                            price_history.quantity = None;
                                        } else {
                                            old_card.newest_quantity = new_card_quantity;
                                        }
                                        old_card.card_history.price_history.push(price_history);

                                        iterate_new_card = true;
                                        break;
                                    }
                                    Ordering::Greater => {
                                        new_cards.push(new_card.clone());
                                        iterate_new_card = true;
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else {
                old_card.newest_quantity = 0;
                old_card
                    .card_history
                    .price_history
                    .push(PriceHistoryTracker::new(Some(0), None, None));
            }
        }

        // Now all the cards that already existed in the price history have been updated
        // with the new price data.
        // So next we add the new cards, they first need to be "zero"-extended with all the timestamps before they
        // were added to the collection.
        let prev_timestamps_count = self.timestamps.len();
        let prefix_extend_vec: Vec<PriceHistoryTracker> =
            std::iter::repeat(PriceHistoryTracker::default())
                .take(prev_timestamps_count)
                .collect();
        for mut new_c in new_cards.into_iter() {
            let mut prefix_extend_vec_inst = prefix_extend_vec.clone();
            let ph = new_c.price_history.pop().unwrap();
            prefix_extend_vec_inst.push(ph);
            new_c.price_history = prefix_extend_vec_inst;
            let newest_quantity = new_c.quantity.parse().unwrap();
            self.card_histories.push(CardHistoryAggregate {
                card_history: new_c,
                newest_quantity,
            });
        }
        // Finally add the new timestamp
        self.timestamps.push(timestamp);
    }

    /// Creates an instance from
    pub fn from_collection_history(
        card_histories: Vec<CardHistoryAggregate>,
        timestamps: Vec<String>,
    ) -> Self {
        Self {
            timestamps,
            card_histories,
        }
    }

    pub fn size(&self) -> usize {
        self.card_histories.len()
    }

    pub fn to_csv_string(&mut self) -> String {
        self.card_histories
            .sort_unstable_by(|a, b| a.card_history.id.cmp(&b.card_history.id));
        // Length of a timestamp, e.g. `2023-11-06T083944Z`
        const TIMESTAMP_LEN: usize = 18;
        // Approximate length of the first entry of a card
        // e.g. `106471,1,"Dragonwing Glider",ONE,Rare,false,[1]0.04;0.02`
        const APPROX_FIRST_ENTRY_CARD_LEN: usize = 64;
        // Approximate length of subsequent entries of a card
        // e.g. `0.003;0.03` or `-,-`
        const APPROX_SUBSEQUENT_ENTRY_CARD_LEN: usize = 8;
        // Make a rough approximation of how much memory we need to allocate for the string
        // to prevent many small (but larger and larger) allocations/reallocations+copy/move
        let approx_prealloc = (self.timestamps.len() * TIMESTAMP_LEN)
            + (self.timestamps.len() * APPROX_SUBSEQUENT_ENTRY_CARD_LEN)
            + (self.card_histories.len() * APPROX_FIRST_ENTRY_CARD_LEN);
        let mut timestamp_row = String::with_capacity(approx_prealloc);

        for ts in self.timestamps.iter() {
            if timestamp_row.is_empty() {
                timestamp_row.push_str(ts);
            } else {
                timestamp_row.push(',');
                timestamp_row.push_str(ts);
            }
        }
        timestamp_row.push('\n');

        eprintln!("{timestamp_row}");

        for c in self.card_histories.iter() {
            let s = c.card_history.to_csv_row();
            timestamp_row.push_str(&s);
            timestamp_row.push('\n');
        }
        timestamp_row
    }
}
