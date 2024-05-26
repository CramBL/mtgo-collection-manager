use std::ffi::OsStr;
use std::io;
use std::process;

use crate::mtgogetter_bin;
use crate::util;

// Convenience functions for calling mtgogetter
fn run_mtgogetter<'a, I>(args: I) -> Result<std::process::Output, std::io::Error>
where
    I: IntoIterator<Item = &'a str>,
{
    // If we're in debug mode initialize the mtgogetter binary path relative to a subdirectory of the project root
    if cfg!(debug_assertions) {
        crate::internal_only::dev_try_init_mtgogetter_bin();
    }
    util::run_with_args(mtgogetter_bin(), args)
}

/// Runs a full update of all MTGO data and saves the output to the given directory
///
/// # Arguments
///
/// * `save_to_dir` - Path to the directory to save the output to
pub fn mtgogetter_update_all(save_to_dir: &OsStr) -> Result<process::Output, io::Error> {
    run_mtgogetter([
        "update",
        "--save-to-dir",
        save_to_dir
            .to_str()
            .unwrap_or_else(|| panic!("{save_to_dir:?} is not valid unicode")),
    ])
}

/// Downloads the latest GoatBots price history and saves it to the current directory
pub fn download_goatbots_price_history() -> Result<process::Output, io::Error> {
    run_mtgogetter(["download", "goatbots-price-history"])
}

/// Downloads the latest GoatBots card definitions and saves it to the current directory
pub fn download_goatbots_card_definitions() -> Result<process::Output, io::Error> {
    run_mtgogetter(["download", "goatbots-card-definitions"])
}
