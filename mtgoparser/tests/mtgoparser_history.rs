mod util;

use chrono::NaiveDateTime;
use mtgoparser::mtgo_card::{
    card_history::CardHistory, collection_history::CollectionHistory, MtgoCard,
};
use util::*;

// Utility for the tests to start of with a vector of `MtgoCard`
fn get_3000_mtgo_card_vector() -> Vec<MtgoCard> {
    match mtgoparser::parse_full(
        FULL_TRADELIST_MEDIUM_FULL_PATH.as_path(),
        SCRYFALL_FULL_PATH.as_path(),
        CARD_DEFINITIONS_FULL_PATH.as_path(),
        PRICE_HISTORY_FULL_PATH.as_path(),
        None,
    ) {
        Ok(cards) => {
            assert_eq!(3000, cards.len());
            cards
        }
        Err(e) => {
            panic!("MTGO Parser error: {e}")
        }
    }
}

#[test]
pub fn test_parse_to_history() -> TestResult {
    let card_histories: Vec<CardHistory> = get_3000_mtgo_card_vector()
        .into_iter()
        .map(|c| CardHistory::from_mtgo_card(c))
        .collect();
    assert_eq!(card_histories.len(), 3000);

    let timestamp =
        NaiveDateTime::parse_from_str("2023-11-06T083944Z", "%Y-%m-%dT%H%M%SZ").unwrap();
    let timestamp_str = timestamp.format("%Y-%m-%dT%H%M%SZ");
    eprintln!("Timestamp: {timestamp_str}");
    for c in card_histories.iter() {
        eprintln!("{}", c.to_csv_row());
    }

    let mut collection_history = CollectionHistory::from_card_history(
        vec![timestamp_str.to_string()],
        card_histories.clone(),
    );

    let timestamp2 = timestamp.checked_add_days(chrono::Days::new(1)).unwrap();
    let timestamp2_str = timestamp.format("%Y-%m-%dT%H%M%SZ");
    let collection_history2 =
        CollectionHistory::from_card_history(vec![timestamp2_str.to_string()], card_histories);

    let mut new_different_cards = get_3000_mtgo_card_vector();
    new_different_cards.get_mut(3).unwrap().id = 7;

    new_different_cards.get_mut(2).unwrap().goatbots_price = 1234.5;

    new_different_cards.get_mut(2).unwrap().goatbots_price = 1234.5;
    new_different_cards.get_mut(2).unwrap().scryfall_price = None;

    collection_history.add_new_card_data(timestamp2_str.to_string(), new_different_cards);

    let csv = collection_history.to_csv_string();
    eprintln!("{csv}");

    for (i, l) in csv.lines().enumerate() {
        eprintln!("{i}: {l}");
    }
    Ok(())
}
