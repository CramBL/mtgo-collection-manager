use super::{APP_DATA_DIR, CURRENT_FULL_TRADE_LIST};
use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;

/// Get the path to the appdata directory
///
/// # Errors
///
/// * If the path to the appdata directory cannot be determined
/// * If the path to the appdata directory doesn't exist
pub fn appdata_path() -> io::Result<PathBuf> {
    let mut appdata_dir = std::env::current_exe()?;
    log::debug!("Path to executable: {appdata_dir:?}");
    appdata_dir.pop();
    if cfg!(windows) {
        appdata_dir.push(format!(r#"{APP_DATA_DIR}\"#));
    } else {
        appdata_dir.push(format!(r#"{APP_DATA_DIR}/"#));
    }
    log::debug!("Path to appdata dir: {appdata_dir:?}");

    let is_exists = match appdata_dir.try_exists() {
        Ok(is_exists) => is_exists,
        Err(e) => {
            log::warn!("Failed to check if app data path exists: {e}");
            return Err(e);
        }
    };

    if !is_exists {
        log::info!("App data path doesn't exist! - {appdata_dir:?}");
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("App data path {APP_DATA_DIR} doesn't exist!"),
        ));
    }

    Ok(appdata_dir)
}

/// Copy the given full trade list to the appdata directory
///
/// # Arguments
///
/// * `full_trade_list_path` - [OsStr] path to the full trade list
///
/// # Errors
///
/// * If the full trade list cannot be copied to the appdata directory
pub fn copy_tradelist_to_appdata(full_trade_list_path: &OsStr) -> io::Result<()> {
    let mut appdata_dir = crate::appdata::util::appdata_path()?;
    appdata_dir.push(CURRENT_FULL_TRADE_LIST);
    std::fs::copy(full_trade_list_path, &appdata_dir)?;
    Ok(())
}

/// Get the path to the current full trade list in the appdata directory if it exists.
/// Returns [None] if the file doesn't exist.
///
/// # Errors
///
/// * If the path to the appdata directory cannot be determined
pub fn current_tradelist_path() -> io::Result<Option<PathBuf>> {
    let mut appdata_dir = crate::appdata::util::appdata_path()?;
    appdata_dir.push(CURRENT_FULL_TRADE_LIST);
    if appdata_dir.try_exists()? && appdata_dir.is_file() {
        Ok(Some(appdata_dir))
    } else {
        log::info!("Current full trade list doesn't exist at: {appdata_dir:?}");
        Ok(None)
    }
}
