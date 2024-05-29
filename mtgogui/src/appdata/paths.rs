use std::{
    io::Error,
    path::{Path, PathBuf},
};

use crate::util::first_file_match_from_dir;

/// [CardDataPaths] contains the paths to the card data files
///
/// The card data files are:
/// * Scryfall data JSON
/// * Card definition JSON
/// * Price history JSON
#[derive(Debug)]
pub struct CardDataPaths {
    scryfall: PathBuf,
    card_definitions: PathBuf,
    price_history: PathBuf,
}

impl CardDataPaths {
    // Strings used to locate the card data files in the appdata directory
    const FIND_SCRYFALL: &'static str = "scryfall-cards";
    const FIND_CARD_DEFINITIONS: &'static str = "card-def";
    const FIND_PRICE_HISTORY: &'static str = "price-his";

    /// Find the paths to the card data files in the appdata directory
    ///
    /// # Arguments
    ///
    /// * `appdata_dir` - The path to the appdata directory
    ///
    /// # Returns
    ///
    /// A [CardDataPaths] instance containing the paths to the card data files
    ///
    /// # Errors
    ///
    /// If any of the card data files are not found, an error is returned describing which files were not found
    pub fn find(appdata_dir: &Path) -> Result<Self, Error> {
        // Try to find the card data files in the appdata directory
        // If any of the files are not found, return an error
        let mut find_errs = Vec::new();

        let scryfall_path: Option<PathBuf> =
            first_file_match_from_dir(Self::FIND_SCRYFALL, appdata_dir, None)?;
        if scryfall_path.is_none() {
            log::info!("Could not locate Scryfall data json in {appdata_dir:?}");
            find_errs.push(format!(
                "Could not find Scryfall data JSON in {appdata_dir:?}"
            ));
        }

        let card_definitions_path: Option<PathBuf> =
            first_file_match_from_dir(Self::FIND_CARD_DEFINITIONS, appdata_dir, None)?;
        if card_definitions_path.is_none() {
            log::info!("Could not locate card definition json in {appdata_dir:?}");
            find_errs.push(format!(
                "Could not find card definition JSON in {appdata_dir:?}"
            ));
        }

        let price_history_path: Option<PathBuf> =
            first_file_match_from_dir(Self::FIND_PRICE_HISTORY, appdata_dir, None)?;
        if price_history_path.is_none() {
            log::info!("Could not locate price history json in {appdata_dir:?}");
            find_errs.push(format!(
                "Could not find price history JSON in {appdata_dir:?}"
            ));
        }

        // If any of the files were not found, return an error describing which files were not found
        if !find_errs.is_empty() {
            return Err(Error::new(
                std::io::ErrorKind::NotFound,
                find_errs.join("\n"),
            ));
        }

        Ok(CardDataPaths {
            scryfall: scryfall_path.unwrap(),
            card_definitions: card_definitions_path.unwrap(),
            price_history: price_history_path.unwrap(),
        })
    }

    /// Get the path to the scryfall data JSON-file
    pub fn scryfall_path(&self) -> &Path {
        self.scryfall.as_path()
    }

    /// Get the path to the card definitions JSON-file
    pub fn card_definitions_path(&self) -> &Path {
        self.card_definitions.as_path()
    }

    /// Get the path to the price history JSON-file
    pub fn price_history_path(&self) -> &Path {
        self.price_history.as_path()
    }
}
