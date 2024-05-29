use chrono::{Datelike, SecondsFormat, Timelike};
use mtgoparser_rs::{
    collection::Collection,
    mtgo_card::{card_history::CardHistory, collection_history::CollectionHistory},
    xml::parse_dek_xml,
};
use parse_goatbots::{
    card_definitions::parse_card_def_json, price_history::parse_price_history_json,
};
use parse_scryfall::ScryfallCard;
use pretty_assertions::assert_eq;
use std::{
    fs,
    path::{Path, PathBuf},
};
use testresult::TestResult;

#[test]
pub fn test_collection_parse_small() -> TestResult {
    let xml_cards = parse_dek_xml(Path::new(
        r"../test/test-data/mtgo/Full Trade List-small-5cards.dek",
    ))?;
    assert_eq!(xml_cards.len(), 5);

    let price_hist = parse_price_history_json(Path::new(
        r"../test/test-data/goatbots/price-hist-small-5cards.json",
    ))?;
    assert_eq!(price_hist.len(), 5);

    let goatbots_card_defs = parse_card_def_json(Path::new(
        r"../test/test-data/goatbots/card-defs-small-5cards.json",
    ))?;
    assert_eq!(goatbots_card_defs.len(), 5);

    let scryfall_json_str =
        fs::read_to_string("../test/test-data/scryfall/default-cards-small-5cards.json")?;
    let scryfall_cards: Vec<ScryfallCard> = serde_json::from_str(&scryfall_json_str)?;
    assert_eq!(scryfall_cards.len(), 5);

    let mut collection = Collection::from_xml_cards(xml_cards)?;

    collection.extract_goatbots_info(goatbots_card_defs, price_hist)?;

    collection.extract_scryfall_info(scryfall_cards)?;

    assert_eq!(collection.total_cards(), 457);

    assert_eq!(
        collection,
        serde_json::from_str(&serde_json::to_string(&collection)?)?
    );

    // Now try aggregating two collections
    // Save the same collection to JSON twice with different timestamps
    // Then aggregate the two collections
    let json_str0 = serde_json::to_string(&collection)?;
    let json_str1 = serde_json::to_string(&collection)?;

    // Save to files
    let subdir = PathBuf::from("collection-history-small-collection-parse");
    let f1 = subdir.join("mtgo_cards_2023-11-05T152700Z");
    let f2 = subdir.join("mtgo_cards_2023-11-05T152800Z");

    fs::create_dir_all(&subdir)?;
    fs::write(&f1, json_str0)?;
    fs::write(&f2, json_str1)?;

    let mut files = mtgoparser_rs::util::get_files_with_timestamp(&subdir)?;
    assert_eq!(files.len(), 2);

    files.sort_by_key(|(_, dt)| std::cmp::Reverse(dt.timestamp()));
    let (path, timestamp) = &files[0];
    assert_eq!(
        path,
        Path::new("collection-history-small-collection-parse/mtgo_cards_2023-11-05T152800Z")
    );
    assert_eq!(timestamp.year(), 2023);
    assert_eq!(timestamp.month(), 11);
    assert_eq!(timestamp.day(), 5);
    assert_eq!(timestamp.hour(), 15);
    assert_eq!(timestamp.minute(), 28);
    assert_eq!(timestamp.second(), 0);

    let (path, timestamp) = &files[1];
    assert_eq!(
        path,
        Path::new("collection-history-small-collection-parse/mtgo_cards_2023-11-05T152700Z")
    );
    assert_eq!(timestamp.minute(), 27);

    // Copy of the most recent filestamp
    let most_recent_ts = timestamp.clone();

    let colletion_from_f1: Collection = serde_json::from_str(&fs::read_to_string(&f1)?)?;
    let mut colletion_from_f2 = serde_json::from_str(&fs::read_to_string(&f2)?)?;

    assert_eq!(colletion_from_f1, colletion_from_f2);
    assert_eq!(colletion_from_f2.total_cards(), 457);

    let mut card_history: Vec<CardHistory> = Vec::new();
    for card in colletion_from_f2.take_cards() {
        card_history.push(CardHistory::from_mtgo_card(card));
    }
    assert_eq!(card_history.len(), 5);

    let collection_history = CollectionHistory::from_card_history(
        most_recent_ts.to_rfc3339_opts(SecondsFormat::Secs, true),
        card_history,
    );
    assert_eq!(collection_history.size(), 5);

    //cleanup
    fs::remove_dir_all(&subdir)?;

    Ok(())
}

#[test]
pub fn test_collection_parse_medium() -> TestResult {
    let xml_cards = parse_dek_xml(Path::new(
        r"../test/test-data/mtgo/Full Trade List-medium-3000cards.dek",
    ))?;
    assert_eq!(xml_cards.len(), 3000);

    let price_hist = parse_price_history_json(Path::new(
        r"../test/test-data/goatbots/price-history-2023-10-02-full.json",
    ))?;
    assert_eq!(price_hist.len(), 76070);

    let goatbots_card_defs = parse_card_def_json(Path::new(
        r"../test/test-data/goatbots/card-definitions-2023-10-02-full.json",
    ))?;
    assert_eq!(goatbots_card_defs.len(), 76070);

    let scryfall_json_str =
        fs::read_to_string("../test/test-data/mtgogetter-out/scryfall-20231002-full.json")?;
    let scryfall_cards: Vec<ScryfallCard> = serde_json::from_str(&scryfall_json_str)?;
    assert_eq!(scryfall_cards.len(), 43705);

    let mut collection = Collection::from_xml_cards(xml_cards)?;

    collection.extract_goatbots_info(goatbots_card_defs, price_hist)?;

    collection.extract_scryfall_info(scryfall_cards)?;

    assert_eq!(collection.unique_cards(), 3000);
    assert_eq!(collection.total_cards(), 8859);

    assert_eq!(
        collection,
        serde_json::from_str(&serde_json::to_string(&collection)?)?
    );
    Ok(())
}

#[test]
fn test_collection_parse_medium_prices() -> TestResult {
    let xml_cards = parse_dek_xml(Path::new(
        r"../test/test-data/mtgo/Full Trade List-medium-3000cards.dek",
    ))?;
    assert_eq!(xml_cards.len(), 3000);

    let price_hist = parse_price_history_json(Path::new(
        r"../test/test-data/goatbots/price-history-2023-10-02-full.json",
    ))?;
    assert_eq!(price_hist.len(), 76070);

    let goatbots_card_defs = parse_card_def_json(Path::new(
        r"../test/test-data/goatbots/card-definitions-2023-10-02-full.json",
    ))?;
    assert_eq!(goatbots_card_defs.len(), 76070);

    let scryfall_json_str =
        fs::read_to_string("../test/test-data/mtgogetter-out/scryfall-20231002-full.json")?;
    let scryfall_cards: Vec<ScryfallCard> = serde_json::from_str(&scryfall_json_str)?;
    assert_eq!(scryfall_cards.len(), 43705);

    let mut collection = Collection::from_xml_cards(xml_cards)?;

    collection.extract_goatbots_info(goatbots_card_defs, price_hist)?;

    collection.extract_scryfall_info(scryfall_cards)?;

    assert_eq!(collection.unique_cards(), 3000);
    assert_eq!(collection.total_cards(), 8859);

    let cards = collection.take_cards();

    // Check the first card is event tickets with expected values
    let expect_tickets = cards.get(0).unwrap();
    assert_eq!(expect_tickets.id, 1);
    assert_eq!(expect_tickets.name, "Event Ticket".into());
    assert_eq!(expect_tickets.goatbots_price, 1.0f32);
    assert_eq!(expect_tickets.scryfall_price.unwrap(), 1.0f32);
    assert_eq!(expect_tickets.quantity, 391);

    let expect_forest_prm = cards.get(10).unwrap();
    assert_eq!(expect_forest_prm.id, 302);
    assert_eq!(expect_forest_prm.quantity, 4);
    assert_eq!(
        expect_forest_prm.name,
        "Forest".to_string().into_boxed_str()
    );
    assert_eq!(expect_forest_prm.set, "PRM".to_string().into_boxed_str());
    assert_eq!(expect_forest_prm.foil, false);
    assert_eq!(expect_forest_prm.goatbots_price, 0.1f32);
    assert_eq!(expect_forest_prm.scryfall_price, Some(0.21f32));

    Ok(())
}
