use std::{
    default,
    io::Error,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime},
};

use flexi_logger::{
    Cleanup, Criterion, Duplicate, FileSpec, LogSpecification, Logger, LoggerHandle, Naming,
    WriteMode,
};

/// Returns the X and Y coordinates of the center of the screen
pub fn center() -> (i32, i32) {
    (
        (fltk::app::screen_size().0 / 2.0) as i32,
        (fltk::app::screen_size().1 / 2.0) as i32,
    )
}

/// Find the first file in the given directory that contains the given string
///
/// # Arguments
///
/// * `f_name` - The string to search for in the file names
/// * `path` - The path to the directory to search in
/// * `max_file_age_secs` - If set, only files younger than this many seconds will be considered
///
/// # Returns
///
/// The path to the first file that contains the given string, or [None] if no such file was found
///
/// # Errors
///
/// * If the given path is not a directory
/// * If the given path cannot be read
/// * If the metadata of a file in the given directory cannot be read (permissions)
/// * If the last modified time of a file in the given directory cannot be read
/// * If the last modified time of a file in the given directory is in the future (very unlikely, but possible because of system clock drift)
///
/// # Example
/// ```
/// # use std::path::Path;
/// # use mtgoupdater::util::first_file_match_from_dir;
///
///  let cwd = std::env::current_dir().unwrap();
///  let first_match = first_file_match_from_dir("Cargo.lock", &cwd, None);
///
///  assert_eq!(
///      PathBuf::from("Cargo.lock"),
///      first_match.unwrap().unwrap().file_name().unwrap()
///  );
/// ```
pub fn first_file_match_from_dir(
    f_name: &str,
    path: &Path,
    max_file_age_secs: Option<u64>,
) -> Result<Option<PathBuf>, Error> {
    for entry in path.read_dir()? {
        let dir_entry = entry?;

        let metadata = std::fs::metadata(&dir_entry.path())?;
        let last_modified = metadata
            .modified()?
            .elapsed()
            .unwrap_or_else(|_| Duration::from_nanos(1)) // If the file was modified in the future, pretend it was modified 1 nanosecond ago
            .as_secs();

        if metadata.is_file() {
            if let Some(max_file_age) = max_file_age_secs {
                if last_modified > max_file_age {
                    continue;
                }
            }

            if dir_entry.file_name().to_string_lossy().contains(f_name) {
                return Ok(Some(dir_entry.path()));
            }
        }
    }

    Ok(None)
}

/// Setup the logger
///
/// Returns a handle to the logger which has to stay alive for the duration of the program
pub fn setup_logger() -> LoggerHandle {
    // Get the path to the MTGO GUI executable
    let mut appdata_dir = std::env::current_exe().unwrap();
    // Remove the executable from the path, then the path points at the directory
    appdata_dir.pop();
    // Add the appdata directory to the path
    appdata_dir.push("appdata");
    // Add the log_files directory to the path
    appdata_dir.push("log_files");

    // 5 MiB
    const MAX_LOG_FILE_SIZE: u64 = 5 * 1024 * 1024;

    let env_log_verbosity_setting = std::env::var("MCM_VERBOSITY").unwrap_or("info".into());
    let log_spec: LogSpecification = match env_log_verbosity_setting.to_lowercase().as_str() {
        "error" => LogSpecification::error(),
        "warn" | "warning" => LogSpecification::warn(),
        "info" => LogSpecification::info(),
        "debug" => LogSpecification::debug(),
        "trace" => LogSpecification::trace(),
        "none" | "off" => LogSpecification::off(),
        _ => panic!("Invalid MCM_VERBOSITY: {env_log_verbosity_setting}"),
    };

    eprintln!("Initiating logger with verbosity={env_log_verbosity_setting}");

    Logger::with(log_spec)
        // Log to a file in the appdata directory
        .log_to_file(
            FileSpec::default()
                .directory(appdata_dir)
                .basename("mcm_log"),
        )
        .rotate(
            // If the program runs long enough:
            // - create a new file every day
            Criterion::Size(MAX_LOG_FILE_SIZE),
            // - let the rotated files have a timestamp in their name
            Naming::Timestamps,
            // - keep at most 7 log files (7 + current log file)
            Cleanup::KeepLogFiles(7),
        )
        // - write all log messages to stderr as well as the file
        .duplicate_to_stderr(Duplicate::All)
        // Configure for asynchronous logging
        .write_mode(WriteMode::Async)
        .start()
        .expect("Failed to initialize logger")
}

/// Describes the relative position and size of a widget in percentage 0-100%.
///
/// Can be used to update the position and size of a widget relative to its current position and size.
#[derive(Debug, Clone, Copy)]
pub struct RelativeSize {
    pub perc_rel_pos_x: i32,
    pub perc_rel_pos_y: i32,
    pub perc_rel_size_w: i32,
    pub perc_rel_size_h: i32,
}
impl RelativeSize {
    /// Create a new [RelativeSize] instance
    pub fn new(
        perc_rel_pos_x: i32,
        perc_rel_pos_y: i32,
        perc_rel_size_w: i32,
        perc_rel_size_h: i32,
    ) -> Self {
        Self {
            perc_rel_pos_x,
            perc_rel_pos_y,
            perc_rel_size_w,
            perc_rel_size_h,
        }
    }

    /// Get the relative value to the given value. e.g. if the relative value percentage
    /// is 50% and the given value is 100, the relative value will be 50.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to get the relative value of
    ///
    /// # Returns
    ///
    /// The relative value of the given value.
    pub fn rel_val_x(&self, x: i32) -> i32 {
        (x * self.perc_rel_pos_x) / 100
    }

    /// Get the relative value to the given value. e.g. if the relative value percentage
    /// is 50% and the given value is 100, the relative value will be 50.
    ///
    /// # Arguments
    ///
    /// * `y` - The value to get the relative value of
    ///
    /// # Returns
    ///
    /// The relative value of the given value.
    pub fn rel_val_y(&self, y: i32) -> i32 {
        (y * self.perc_rel_pos_y) / 100
    }

    /// Get the relative value to the given value. e.g. if the relative value percentage
    /// is 50% and the given value is 100, the relative value will be 50.
    ///
    /// # Arguments
    ///
    /// * `w` - The value to get the relative value of
    ///
    /// # Returns
    ///
    /// The relative value of the given value.
    pub fn rel_val_w(&self, w: i32) -> i32 {
        (w * self.perc_rel_size_w) / 100
    }

    /// Get the relative value to the given value. e.g. if the relative value percentage
    /// is 50% and the given value is 100, the relative value will be 50.
    ///
    /// # Arguments
    ///
    /// * `h` - The value to get the relative value of
    ///
    /// # Returns
    ///
    /// The relative value of the given value.
    pub fn rel_val_h(&self, h: i32) -> i32 {
        (h * self.perc_rel_size_h) / 100
    }
}

impl default::Default for RelativeSize {
    fn default() -> Self {
        Self::new(100, 100, 100, 100)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_match_cargolock() {
        let cwd = std::env::current_dir().unwrap();
        let first_match = first_file_match_from_dir("Cargo.lock", &cwd, None);

        assert_eq!(
            PathBuf::from("Cargo.lock"),
            first_match.unwrap().unwrap().file_name().unwrap()
        );
    }

    #[test]
    fn test_find_first_match_cargotoml() {
        let cwd = std::env::current_dir().unwrap();
        let first_match = first_file_match_from_dir("Cargo.toml", &cwd, None);

        assert_eq!(
            PathBuf::from("Cargo.toml"),
            first_match.unwrap().unwrap().file_name().unwrap()
        );
    }

    #[test]
    fn test_find_first_match_cargo() {
        // Searching for "Cargo" can find either Cargo.lock or Cargo.toml

        let cwd = std::env::current_dir().unwrap();
        let first_match = first_file_match_from_dir("Cargo", &cwd, None);

        let path = first_match.unwrap().unwrap();
        let name = path.file_name().unwrap();

        if name != "Cargo.lock" && name != "Cargo.toml" {
            panic!("Did not get Cargo.lock or Cargo.toml, got: {name:?}")
        }
    }
}
