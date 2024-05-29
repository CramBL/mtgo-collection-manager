use super::card_history::CardHistory;

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
    pub fn from_card_history(timestamp: String, card_history: Vec<CardHistory>) -> Self {
        let mut card_histories: Vec<CardHistoryAggregate> = vec![];
        for cardh in card_history.into_iter() {
            let mut newest_quantity = 0;
            for (quant, _, _) in cardh.price_history.iter().rev() {
                if let Some(q) = quant {
                    newest_quantity = *q;
                    break;
                }
            }
            card_histories.push(CardHistoryAggregate::new(cardh, newest_quantity));
        }

        Self {
            timestamps: vec![timestamp],
            card_histories,
        }
    }

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
}
