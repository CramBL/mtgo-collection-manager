#![allow(dead_code)]
/// Common imports/use for tests
use once_cell::sync::Lazy;

#[allow(unused_imports)]
pub use {
    std::{
        fs,
        path::{Path, PathBuf},
    },
    testresult::TestResult,
};

#[allow(unused_imports)]
use pretty_assertions::{assert_eq, assert_ne, assert_str_eq};

pub static SCRYFALL_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/mtgogetter-out/scryfall-20231002-full.json"));
pub static SCRYFALL_SMALL_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/scryfall/default-cards-small-5cards.json"));
pub static CARD_DEFINITIONS_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/goatbots/card-definitions-2023-10-02-full.json"));
pub static CARD_DEFINITIONS_SMALL_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/goatbots/card-defs-small-5cards.json"));
pub static PRICE_HISTORY_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/goatbots/price-history-2023-10-02-full.json"));
pub static PRICE_HISTORY_SMALL_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/goatbots/price-hist-small-5cards.json"));
pub static FULL_TRADELIST_MEDIUM_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from("../test/test-data/mtgo/Full Trade List-medium-3000cards.dek"));
pub static FULL_TRADELIST_SMALL_FULL_PATH: Lazy<PathBuf> =
    Lazy::new(|| PathBuf::from(r"../test/test-data/mtgo/Full Trade List-small-5cards.dek"));
