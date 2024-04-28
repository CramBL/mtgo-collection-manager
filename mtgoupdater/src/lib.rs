#![allow(dead_code)]

use std::error;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

pub mod date;
pub mod internal_only;
pub mod mtgo_preprocessor_api;
pub mod mtgogetter_api;
mod util;
mod zip_util;

pub use mtgogetter_api::download_custom_url as get_custom_url;
pub use mtgogetter_api::download_goatbots_card_definitions as get_goatbots_card_definitions;
pub use mtgogetter_api::download_goatbots_price_history as get_goatbots_price_history;
use zip_util::Archive;
use zip_util::Archived;
use zip_util::UnArchived;

static MTGOGETTER_BIN: OnceLock<OsString> = OnceLock::new();
static MTGOPARSER_BIN: OnceLock<OsString> = OnceLock::new();

pub use mtgoparser_rs::mtgo_card::MtgoCard;
pub use mtgoparser_rs::mtgo_card::Rarity;
pub use mtgoparser_rs::parse_full;

/// Returns the version of `MTGO Updater`
pub fn mtgo_updater_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Sets the path to the `MTGO Getter` binary
///
/// # Arguments
///
/// * `bin_path` - Path to the `MTGO Getter` binary
///
/// # Errors
///
/// Returns an error if path has already been set.
pub fn set_mtgogetter_bin(bin_path: OsString) -> Result<(), OsString> {
    MTGOGETTER_BIN.set(bin_path)
}

/// Sets the path to the binary of `MTGO Parser`/`MTGO Preprocessor`
///
/// # Arguments
///
/// * `bin_path` - Path to the `MTGO Parser`/`MTGO Preprocessor` binary
///
/// # Errors
///
/// Returns an error if path has already been set.
pub fn set_mtgoparser_bin(bin_path: OsString) -> Result<(), OsString> {
    MTGOPARSER_BIN.set(bin_path)
}

/// Gets the path to the `MTGO Getter` binary
///
/// If the path has not been set, it will be set to the default path relative to the current executable
///
/// # Panics
///
/// Panics if the current executable path cannot be determined
pub(crate) fn mtgogetter_bin() -> &'static OsStr {
    MTGOGETTER_BIN.get_or_init(|| {
        let mut path = std::env::current_exe().expect("Failed to get current executable path");
        path.pop();
        path.push("bin");
        path.push("mtgogetter");
        if cfg!(windows) {
            path.set_extension(std::env::consts::EXE_EXTENSION);
        }
        path.into_os_string()
    })
}

/// Gets the path to the `MTGO Parser`/`MTGO Preprocessor` binary
///
/// If the path has not been set, it will be set to the default path relative to the current executable
///
/// # Panics
///
/// Panics if the current executable path cannot be determined
pub(crate) fn mtgoparser_bin() -> &'static OsStr {
    MTGOPARSER_BIN.get_or_init(|| {
        let mut path = std::env::current_exe().expect("Failed to get current executable path");
        path.pop();
        path.push("bin");
        path.push("mtgo_preprocessor");
        if cfg!(windows) {
            path.set_extension(std::env::consts::EXE_EXTENSION);
        }
        path.into_os_string()
    })
}

/// Finds all price history JSON-files (pattern `mtgo-cards_YYYY-MM-DDTHHMMSSZ`) in
/// the given directory and compresses them into a ZIP-file with the given name.
/// Deletes the JSON-files after they have been added to the ZIP-file.
///
/// If a ZIP-file with the same name already exists, the JSON-files are added to the existing ZIP-file.
///
/// # Arguments
/// * `dir` - The directory to search for JSON-files
/// * `zip_file` - The name of the ZIP-file to create or add to
///
/// # Errors
///
/// Returns an error if the ZIP-file cannot be created or if the JSON-files cannot be added to the ZIP-file.
pub fn zip_price_history(
    dir: impl AsRef<Path>,
    zip_file: &str,
) -> Result<(), Box<dyn error::Error>> {
    let mut json_files = Vec::new();

    // Find all JSON-files in the given directory with the pattern `mtgo-cards_YYYY-MM-DDTHHMMSSZ`
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|fname| fname.to_str()) {
                if name.starts_with("mtgo-cards_") {
                    json_files.push(path);
                }
            }
        }
    }

    // Create ZIP-file if it doesn't exist
    let zip_file = Path::new(zip_file);
    if zip_file.exists() {
        let mut archive = Archive::<Archived>::init(zip_file);
        // Add all JSON-files to the ZIP-file
        archive.move_to_archive(json_files.iter().map(|p| p.as_path()))?;
    } else {
        let mut archive = Archive::<UnArchived>::new(zip_file);
        // Add all JSON-files to the ZIP-file
        json_files.into_iter().for_each(|f| archive.move_file(f));
        // Create ZIP-file
        archive.archive()?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;
    use temp_dir::TempDir;

    /// Test zip_price_history()
    ///
    /// 1. Create a temporary directory and create two JSON-files in it
    /// 2. Call zip_price_history() with the temporary directory and a ZIP-file name
    /// 3. Check that the ZIP-file exists and contains the two JSON-files
    /// 4. Call zip_price_history() again with the temporary directory and the same ZIP-file name
    /// 5. Check that the ZIP-file exists and contains the two JSON-files as in step 3
    /// 6. Add a third JSON-file to the temporary directory
    /// 7. Call zip_price_history() again with the temporary directory and the same ZIP-file name
    /// 8. Check that the ZIP-file exists and contains the three JSON-files twice
    #[test]
    fn test_zip_price_history() {
        struct TestFile {
            name: &'static str,
            contents: &'static str,
        }
        let first_2_json_files = vec![
            TestFile {
                name: "mtgo-cards_2020-11-06T083944Z.json",
                contents: r#"{"name":"Test1"}"#,
            },
            TestFile {
                name: "mtgo-cards_2020-11-06T115147Z.json",
                contents: r#"{"name":"Test2"}"#,
            },
        ];
        let third_json_file = TestFile {
            name: "mtgo-cards_2020-11-08T084732Z.json",
            contents: r#"{"name":"Test3"}"#,
        };

        // 1. Create a temporary directory and create two JSON-files in it
        let temp_dir = TempDir::new().expect("Failed to create temporary directory");
        for file in &first_2_json_files {
            let path = temp_dir.child(file.name);
            fs::write(path, file.contents).expect("Failed to create test file");
        }

        // 2. Call zip_price_history() with the temporary directory and a ZIP-file name
        let zip_file = temp_dir.child("test.zip");
        zip_price_history(temp_dir.path(), zip_file.to_str().unwrap())
            .expect("Failed to zip price history");

        // 3. Check that the ZIP-file exists and contains the two JSON-files
        assert!(zip_file.exists());
        let mut expected_archive =
            zip::ZipArchive::new(fs::File::open(&zip_file).unwrap()).unwrap();
        assert_eq!(expected_archive.len(), first_2_json_files.len());
        for file in &first_2_json_files {
            let mut archive_file = expected_archive.by_name(file.name).unwrap();
            let mut contents = String::new();
            archive_file.read_to_string(&mut contents).unwrap();
            assert_eq!(contents, file.contents);
        }

        // Check that the directory no longer contains the two JSON-files
        for file in &first_2_json_files {
            let path = temp_dir.child(file.name);
            assert!(!path.exists());
        }

        // 4. Call zip_price_history() again with the temporary directory and the same ZIP-file name
        // should be a no-op
        zip_price_history(temp_dir.path(), zip_file.to_str().unwrap())
            .expect("Failed to zip price history");

        // 5. Check that the ZIP-file exists and contains the two JSON-files as in step 3
        assert!(zip_file.exists());
        let mut expected_archive =
            zip::ZipArchive::new(fs::File::open(&zip_file).unwrap()).unwrap();
        assert_eq!(expected_archive.len(), first_2_json_files.len());
        for file in &first_2_json_files {
            let mut archive_file = expected_archive.by_name(file.name).unwrap();
            let mut contents = String::new();
            archive_file.read_to_string(&mut contents).unwrap();
            assert_eq!(contents, file.contents);
        }

        // 6. Add a third JSON-file to the temporary directory
        let path = temp_dir.child(third_json_file.name);
        fs::write(path, third_json_file.contents).expect("Failed to create test file");

        // 7. Call zip_price_history() again with the temporary directory and the same ZIP-file name
        zip_price_history(temp_dir.path(), zip_file.to_str().unwrap())
            .expect("Failed to zip price history");

        // 8. Check that the ZIP-file exists and contains the three JSON-files twice
        assert!(zip_file.exists());
        let mut expected_archive =
            zip::ZipArchive::new(fs::File::open(&zip_file).unwrap()).unwrap();
        assert_eq!(
            expected_archive.len(),
            first_2_json_files.len() + 1 // 1 for the third JSON-file
        );
        // Check that the ZIP-file contains the first 2 JSON-files
        for file in &first_2_json_files {
            let mut archive_file = expected_archive.by_name(file.name).unwrap();
            let mut contents = String::new();
            archive_file.read_to_string(&mut contents).unwrap();
            assert_eq!(contents, file.contents);
        }
        // Check that the ZIP-file contains the third JSON-file
        let mut archive_file = expected_archive.by_name(third_json_file.name).unwrap();
        let mut contents = String::new();
        archive_file.read_to_string(&mut contents).unwrap();
        assert_eq!(contents, third_json_file.contents);
    }
}
