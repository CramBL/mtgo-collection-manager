use std::{ffi::OsStr, io};

use mtgoparser_rs::mtgo_card::MtgoCard;

use crate::{mtgoparser_bin, util};

/// Convenience functions for calling MTGO Parser/MTGO Preprocessor
///
/// # Example
/// ```
/// # use std::path::Path;
/// # use mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor;
/// # use mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_version;
/// # use mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_parse_full;
/// # use mtgoupdater::mtgo_card::MtgoCard;
///
///
/// // Invoke MTGO preprocessor
/// match run_mtgo_preprocessor(["--version"]) {
///    Ok(out) => {
///       eprintln!("stderr:\n{stderr}", stderr = String::from_utf8_lossy(&out.stderr),);
///       eprintln!("stdout:\n{stdout}", stdout = String::from_utf8_lossy(&out.stdout),);
///       assert!(out.status.success());
///       assert!(String::from_utf8_lossy(&out.stdout).contains("v0.1.0"));
///   },
///   Err(e) => panic!("MTGO Preprocessor error: {e}")
/// }
/// ```
pub fn run_mtgo_preprocessor<'a, I>(args: I) -> Result<std::process::Output, std::io::Error>
where
    I: IntoIterator<Item = &'a str>,
{
    // If we're in debug mode initialize the mtgoparser/mtgo_preprocessor binary path relative to a subdirectory of the project root
    if cfg!(debug_assertions) {
        crate::internal_only::dev_try_init_mtgoparser_bin();
    }
    util::run_with_args(mtgoparser_bin(), args)
}

/// Returns the version of `MTGO Parser`/`MTGO Preprocessor`
pub fn run_mtgo_preprocessor_version() -> Result<std::process::Output, std::io::Error> {
    run_mtgo_preprocessor(["--version"])
}

/// Runs a full parse of the MTGO collection and returns stdout deserialized as a [`Vec<MtgoCard>`]
///
/// # Arguments
///
/// * `full_trade_list_path` - Path to the full trade list XML-file
/// * `scryfall_path` - Path to the Scryfall bulk data JSON-file
/// * `card_definitions_path` - Path to the Goatbots card definitions JSON-file
/// * `price_history_path` - Path to the Goatbots price history JSON-file
/// * `save_json_to_dir` - If `Some(dir)`, saves the JSON output to the given directory
///
/// # Example
///
/// ```
/// # use std::path::Path;
///
/// let full_trade_list_path = Path::new("../test/test-data/mtgo/Full Trade List-medium-3000cards.dek");
/// let scryfall_path = Path::new("../test/test-data/mtgogetter-out/scryfall-20231002-full.json");
/// let card_definitions_path = Path::new("../test/test-data/goatbots/card-definitions-2023-10-02-full.json");
/// let price_history_path = Path::new("../test/test-data/goatbots/price-history-2023-10-02-full.json");
///
/// // Invoke MTGO preprocessor
/// match mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_parse_full(
///    full_trade_list_path.as_os_str(),
///    scryfall_path.as_os_str(),
///    card_definitions_path.as_os_str(),
///    price_history_path.as_os_str(),
///    None,
///  ) {
///    Ok(cards) => assert_eq!(3000, cards.len()),
///    Err(e) => panic!("MTGO Preprocessor error: {e}")
/// }
/// ```
///
/// # Panics
///
/// Panics if `full_trade_list_path`, `scryfall_path`, `card_definitions_path`, or `price_history_path` are not valid unicode
///
/// # Errors
///
/// Returns an error if the MTGO Preprocessor binary fails to run or returns an error
pub fn run_mtgo_preprocessor_parse_full(
    full_trade_list_path: &OsStr,
    scryfall_path: &OsStr,
    card_definitions_path: &OsStr,
    price_history_path: &OsStr,
    save_json_to_dir: Option<&OsStr>,
) -> Result<Vec<MtgoCard>, io::Error> {
    let mut args = vec![
        "run",
        "-u",
        "--scryfall-path",
        scryfall_path
            .to_str()
            .expect("scryfall_path is not valid unicode"),
        "--full-trade-list",
        full_trade_list_path
            .to_str()
            .expect("full_trade_list_path is not valid unicode"),
        "--card-definitions",
        card_definitions_path
            .to_str()
            .expect("card_definitions_path is not valid unicode"),
        "--price-history",
        price_history_path
            .to_str()
            .expect("price_history_path is not valid unicode"),
    ];

    if let Some(dir) = save_json_to_dir {
        args.push("--appdata-dir");
        args.push(
            dir.to_str()
                .expect("save to json directory is not valid unicode"),
        );
    }

    let out = run_mtgo_preprocessor(args)?;
    if out.status.success() {
        eprintln!(
            "stderr:\n{stderr}",
            stderr = String::from_utf8_lossy(&out.stderr),
        );

        let stdout_json = String::from_utf8_lossy(&out.stdout);
        Ok(serde_json::from_str(&stdout_json).unwrap())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!(
                "MTGO Preprocessor errored: {}",
                String::from_utf8_lossy(&out.stderr)
            ),
        ))
    }
}
