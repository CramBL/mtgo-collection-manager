use std::{io, path::PathBuf};

use mtgogetter::fetch_all;

/// Runs a full update of all MTGO data and saves the output to the given directory
///
/// # Arguments
///
/// * `save_to_dir` - Path to the directory to save the output to
pub fn new_update_all(save_to_dir: &std::ffi::OsStr) -> Result<(), io::Error> {
    let save_to_dir = PathBuf::from(save_to_dir);
    if !save_to_dir.exists() {
        panic!("Save to dir does not exist");
    }

    fetch_all(save_to_dir)
}
