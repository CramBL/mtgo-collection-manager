use std::path::Path;

use pretty_assertions::assert_eq;

#[test]
fn test_full_parse_3000cards_from_pathbuf() {
    let scryfall_path =
        std::path::PathBuf::from("../test/test-data/mtgogetter-out/scryfall-20231002-full.json");
    let card_definitions_path = std::path::PathBuf::from(
        "../test/test-data/goatbots/card-definitions-2023-10-02-full.json",
    );
    let price_history_path =
        std::path::PathBuf::from("../test/test-data/goatbots/price-history-2023-10-02-full.json");

    let full_trade_list_path =
        std::path::PathBuf::from("../test/test-data/mtgo/Full Trade List-medium-3000cards.dek");

    match mtgoupdater::parse_full(
        full_trade_list_path.as_path(),
        scryfall_path.as_path(),
        card_definitions_path.as_path(),
        price_history_path.as_path(),
        None,
    ) {
        Ok(cards) => {
            eprintln!("MTGO Parser output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            println!("Got {} cards", cards.len());
            assert_eq!(3000, cards.len());
        }
        Err(e) => {
            panic!("MTGO Parser error: {e}")
        }
    }
}

#[test]
fn test_full_parse_3000cards_bad_path() {
    let scryfall_path = Path::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json");
    let card_definitions_path =
        Path::new("../test/test-data/goatbots/card-definitions-2023-10-02-full.json");
    let price_history_path =
        Path::new("../test/test-data/goatbots/price-history-2023-10-02-full.json");

    let full_trade_list_path_bad =
        Path::new("../test/test-data/mtgo/Full Trade List-medium-3000cards.dekx"); // extra x in the end

    // Invoke MTGO parser-rs
    match mtgoupdater::parse_full(
        full_trade_list_path_bad,
        scryfall_path,
        card_definitions_path,
        price_history_path,
        None,
    ) {
        Ok(cards) => {
            eprintln!("MTGO Parser output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            println!("Got {} cards", cards.len());
            panic!("Expected failure with bad path!")
        }
        Err(e) => {
            eprintln!("MTGO Parser error: {e}");
        }
    }
}

#[test]
fn test_full_parse_3000cards_from_path_with_save_to_dir() {
    let local_test_dir = "target/test_full_parse_3000cards_from_path_with_save_to_dir/";
    std::fs::create_dir_all(local_test_dir).unwrap();

    let card_definitions_path = std::path::PathBuf::from(
        "../test/test-data/goatbots/card-definitions-2023-10-02-full.json",
    );
    let price_history_path =
        std::path::PathBuf::from("../test/test-data/goatbots/price-history-2023-10-02-full.json");

    let full_trade_list_path =
        std::path::PathBuf::from("../test/test-data/mtgo/Full Trade List-medium-3000cards.dek");

    let save_to_dir = Path::new(local_test_dir);

    let state_log_path =
        std::path::PathBuf::from("../test/test-data/mtgogetter-out/state_log.toml");
    assert!(state_log_path.exists());
    let mut save_to_dir_state_log = save_to_dir.to_path_buf();
    save_to_dir_state_log.push("state_log.toml");
    _ = std::fs::copy(
        state_log_path.as_os_str(),
        save_to_dir_state_log.as_os_str(),
    )
    .unwrap();

    // Invoke MTGO parser-rs
    match mtgoupdater::parse_full(
        full_trade_list_path.as_path(),
        Path::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json"),
        card_definitions_path.as_path(),
        price_history_path.as_path(),
        Some(save_to_dir),
    ) {
        Ok(cards) => {
            eprintln!("MTGO Parser output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            println!("Got {} cards", cards.len());
            assert_eq!(3000, cards.len());
            // Cleanup
            std::fs::remove_dir_all(local_test_dir).unwrap();
        }
        Err(e) => {
            // Cleanup
            std::fs::remove_dir_all(local_test_dir).unwrap();
            panic!("MTGO Parser error: {e}")
        }
    }
}

// Copies the test example state_log to the json dir and it is then used by the MTGO parser
// deletes it again after test
#[test]
fn test_full_parse_3000cards_from_path_with_save_to_dir_state_log() {
    let local_test_dir = "target/test_full_parse_3000cards_from_path_with_save_to_dir_state_log/";
    std::fs::create_dir_all(local_test_dir).unwrap();

    let card_definitions_path = std::path::PathBuf::from(
        "../test/test-data/goatbots/card-definitions-2023-10-02-full.json",
    );
    let price_history_path =
        std::path::PathBuf::from("../test/test-data/goatbots/price-history-2023-10-02-full.json");

    let full_trade_list_path =
        std::path::PathBuf::from("../test/test-data/mtgo/Full Trade List-medium-3000cards.dek");
    let save_to_dir = Path::new("target/");

    let state_log_path =
        std::path::PathBuf::from("../test/test-data/mtgogetter-out/state_log.toml");
    assert!(state_log_path.exists());
    let mut save_to_dir_state_log = save_to_dir.to_path_buf();
    save_to_dir_state_log.push("state_log.toml");
    _ = std::fs::copy(
        state_log_path.as_os_str(),
        save_to_dir_state_log.as_os_str(),
    )
    .unwrap();

    match mtgoupdater::parse_full(
        full_trade_list_path.as_path(),
        Path::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json"),
        card_definitions_path.as_path(),
        price_history_path.as_path(),
        Some(save_to_dir),
    ) {
        Ok(cards) => {
            eprintln!("MTGO Parser output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            assert_eq!(3000, cards.len());
            // Cleanup
            std::fs::remove_dir_all(local_test_dir).unwrap();
        }
        Err(e) => {
            // Cleanup
            std::fs::remove_dir_all(local_test_dir).unwrap();
            panic!("MTGO Parser error: {e}")
        }
    }
}
