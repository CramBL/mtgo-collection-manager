use std::{ffi::OsString, io, path::PathBuf};

use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use toml::Table;

use super::GUI_STATE;

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq)]
pub struct GuiState {
    tradelist_added_date: Option<DateTime<Utc>>,
}

impl GuiState {
    /// Create a new [GuiState] instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Save the [GuiState] to the given directory as a TOML file.
    ///
    /// # Arguments
    ///
    /// * `dst_dir` - The directory to save the [GuiState] to.
    ///
    /// # Errors
    ///
    /// Returns an [io::Error] if the [GuiState] fails to be saved.
    pub fn save(&self, mut dst_dir: PathBuf) -> io::Result<()> {
        dst_dir.push(GUI_STATE);
        let toml = toml::to_string(&self).expect("Failed to serialize GUI state");
        std::fs::write(dst_dir, toml)?;
        Ok(())
    }

    /// Load the [GuiState] from the given directory containing a TOML file named [GUI_STATE] or create a default [GuiState] if there's no matching file in the directory.
    ///
    /// # Arguments
    ///
    /// * `src_dir` - The directory to load the [GuiState] from.
    ///
    /// # Errors
    ///
    /// Returns an [io::Error] if the [GuiState] fails to be loaded.
    pub fn load(mut src_dir: PathBuf) -> io::Result<Self> {
        src_dir.push(GUI_STATE);
        let gui_state = if src_dir.try_exists()? {
            let toml = std::fs::read_to_string(src_dir)?;
            match toml::from_str(&toml) {
                Ok(gui_state) => gui_state,
                Err(e) => {
                    log::warn!("Failed to deserialize GUI state: {e}");
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("Failed to deserialize GUI state: {e}"),
                    ));
                }
            }
        } else {
            log::info!("No GUI state found, creating default GUI state");
            Self::new()
        };
        Ok(gui_state)
    }

    /// Save the current [`DateTime<Utc>`] as the last time a tradelist was added.
    pub fn new_tradelist(&mut self) {
        self.tradelist_added_date = Some(Utc::now());
    }

    /// Get the [`DateTime<Utc>`] of the last time a tradelist was added.
    pub fn get_tradelist_added_date(&self) -> Option<&DateTime<Utc>> {
        self.tradelist_added_date.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use temp_dir::TempDir;

    #[test]
    fn test_gui_state_load_save() {
        let tmpdir = TempDir::new().unwrap();
        let tmpdir_path = tmpdir.path().to_path_buf();

        let gui_state = GuiState::new();

        gui_state.save(tmpdir_path.clone()).unwrap();
        let gui_state_loaded = GuiState::load(tmpdir_path).unwrap();

        assert_eq!(gui_state, gui_state_loaded);
    }

    #[test]
    fn test_gui_state_tradelist_data() {
        let mut gui_state = GuiState::new();
        let now = Utc::now();
        gui_state.tradelist_added_date = Some(now);

        let after = now + chrono::Duration::seconds(1);

        assert!(gui_state.tradelist_added_date.unwrap() < after);
    }

    #[test]
    fn test_gui_state_tradelist_data_serde() {
        let tmpdir = TempDir::new().unwrap();
        let tmpdir_path = tmpdir.path().to_path_buf();

        let mut gui_state = GuiState::new();
        let now = Utc::now();
        gui_state.tradelist_added_date = Some(now);

        gui_state.save(tmpdir_path.clone()).unwrap();
        let gui_state_loaded = GuiState::load(tmpdir_path).unwrap();

        assert_eq!(gui_state, gui_state_loaded);
        assert_eq!(gui_state_loaded.tradelist_added_date.unwrap(), now);
        assert!(
            gui_state_loaded.tradelist_added_date.unwrap() < now + chrono::Duration::seconds(1)
        );
    }

    #[test]
    fn test_gui_state_tradelist_date_format() {
        let mut gui_state = GuiState::new();
        let date = DateTime::<Utc>::UNIX_EPOCH;
        gui_state.tradelist_added_date = Some(date);
        let simple_readable_date = date.format("%-d %B, %C%y");
        eprintln!("date: {simple_readable_date}");

        assert_eq!(
            gui_state
                .tradelist_added_date
                .unwrap()
                .format("%-d %B, %C%y")
                .to_string(),
            simple_readable_date.to_string()
        );
        assert_eq!(
            gui_state
                .tradelist_added_date
                .unwrap()
                .format("%-d %B, %C%y")
                .to_string(),
            "1 January, 1970"
        );
    }
}
