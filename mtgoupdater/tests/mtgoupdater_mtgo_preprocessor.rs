use std::{ffi::OsStr, path::Path};

use mtgoupdater::internal_only;
use pretty_assertions::assert_eq;

#[test]
fn test_call_mtgo_preprocessor_example_collection_json_stdout() {
    internal_only::dev_try_init_mtgoparser_bin();

    // Check the build directory exists
    assert!(
        std::path::Path::new("../mtgoparser/build").exists(),
        "Build directory does not exist, build mtgoparser before running this test"
    );
    // Check the build src mtgo_preprocessor directory exists
    assert!(
        std::path::Path::new("../mtgoparser/build/src/mtgo_preprocessor").exists(),
        "mtgo_preprocessor directory does not exist, build mtgoparser before running this test"
    );
    // Check the mtgo_preprocessor binary exists
    assert!(
        std::path::Path::new(internal_only::DEV_MTGOPARSER_BIN).exists(),
        "mtgo_preprocessor binary ({mtgoparser_bin}) does not exist, build mtgoparser before running this test", mtgoparser_bin = internal_only::DEV_MTGOPARSER_BIN
    );

    match mtgoupdater::internal_only::run_mtgo_preprocessor_example_collection_json_stdout() {
        Ok(output) => {
            println!("Status:\n{status}", status = output.status,);
            println!(
                "stderr:\n{stderr}",
                stderr = String::from_utf8_lossy(&output.stderr),
            );
            assert!(
                output.status.success(),
                "Process failed with non-zero exit code: {}",
                output.status.code().unwrap_or(123)
            );

            let stdout_json = String::from_utf8_lossy(&output.stdout);
            let deserialized: Vec<mtgoupdater::MtgoCard> =
                serde_json::from_str(&stdout_json).unwrap();
            println!("Got {len} cards as JSON", len = deserialized.len());
            assert_eq!(deserialized.len(), 3000);
            assert_eq!(deserialized[0].id, 1);
            assert_eq!(deserialized[0].quantity, 391);
            assert_eq!(deserialized[0].name, "Event Ticket".into());
        }
        Err(e) => panic!("Unexpected error: {e}"),
    }
}

#[test]
fn test_full_parse_3000cards_from_pathbuf() {
    internal_only::dev_try_init_mtgoparser_bin();

    let scryfall_path =
        std::path::PathBuf::from("../test/test-data/mtgogetter-out/scryfall-20231002-full.json");
    let card_definitions_path = std::path::PathBuf::from(
        "../test/test-data/goatbots/card-definitions-2023-10-02-full.json",
    );
    let price_history_path =
        std::path::PathBuf::from("../test/test-data/goatbots/price-history-2023-10-02-full.json");

    let full_trade_list_path =
        std::path::PathBuf::from("../test/test-data/mtgo/Full Trade List-medium-3000cards.dek");
    // Invoke MTGO preprocessor
    match mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_parse_full(
        full_trade_list_path.as_os_str(),
        scryfall_path.as_os_str(),
        card_definitions_path.as_os_str(),
        price_history_path.as_os_str(),
        None,
    ) {
        Ok(cards) => {
            eprintln!("MTGO Preprocessor output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            println!("Got {} cards", cards.len());
            assert_eq!(3000, cards.len());
        }
        Err(e) => {
            panic!("MTGO Preprocessor error: {e}")
        }
    }
}

#[test]
fn test_full_parse_3000cards_bad_path() {
    internal_only::dev_try_init_mtgoparser_bin();

    let scryfall_path = Path::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json");
    let card_definitions_path =
        Path::new("../test/test-data/goatbots/card-definitions-2023-10-02-full.json");
    let price_history_path =
        Path::new("../test/test-data/goatbots/price-history-2023-10-02-full.json");

    let full_trade_list_path_bad =
        Path::new("../test/test-data/mtgo/Full Trade List-medium-3000cards.dekx"); // extra x in the end

    // Invoke MTGO preprocessor
    match mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_parse_full(
        full_trade_list_path_bad.as_os_str(),
        scryfall_path.as_os_str(),
        card_definitions_path.as_os_str(),
        price_history_path.as_os_str(),
        None,
    ) {
        Ok(cards) => {
            eprintln!("MTGO Preprocessor output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            println!("Got {} cards", cards.len());
            panic!("Expected failure with bad path!")
        }
        Err(e) => {
            eprintln!("MTGO Preprocessor error: {e}");
        }
    }
}

#[test]
fn test_full_parse_3000cards_from_path_with_save_to_dir() {
    internal_only::dev_try_init_mtgoparser_bin();
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

    // Invoke MTGO preprocessor
    match mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_parse_full(
        full_trade_list_path.as_os_str(),
        OsStr::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json"),
        card_definitions_path.as_os_str(),
        price_history_path.as_os_str(),
        Some(save_to_dir.as_os_str()),
    ) {
        Ok(cards) => {
            eprintln!("MTGO Preprocessor output: {} cards", cards.len());
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
            panic!("MTGO Preprocessor error: {e}")
        }
    }
}

// Copies the test example state_log to the json dir and it is then used by the mtgo_preprocessor
// deletes it again after test
#[test]
fn test_full_parse_3000cards_from_path_with_save_to_dir_state_log() {
    internal_only::dev_try_init_mtgoparser_bin();
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

    // Invoke MTGO preprocessor
    match mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_parse_full(
        full_trade_list_path.as_os_str(),
        OsStr::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json"),
        card_definitions_path.as_os_str(),
        price_history_path.as_os_str(),
        Some(save_to_dir.as_os_str()),
    ) {
        Ok(cards) => {
            eprintln!("MTGO Preprocessor output: {} cards", cards.len());
            // Fill the progress bar as appropriate
            // Give all the data to the collection table
            assert_eq!(3000, cards.len());
            // Cleanup
            std::fs::remove_dir_all(local_test_dir).unwrap();
        }
        Err(e) => {
            // Cleanup
            std::fs::remove_dir_all(local_test_dir).unwrap();
            panic!("MTGO Preprocessor error: {e}")
        }
    }
}
