pub mod paths;
pub mod state;
pub mod update;
pub mod util;

/// Directory that stores all collection data
pub const APP_DATA_DIR: &str = "appdata";
/// Name of the file that stores the current full trade list in the appdata directory
pub const CURRENT_FULL_TRADE_LIST: &str = "current-full-trade-list.dek";
/// Name of the file that stores state information for the GUI
pub const GUI_STATE: &str = "gui_state.toml";
/// Name of the file that stores the state log for the MTGO getter
pub const MTGO_GETTER_STATE_LOG: &str = "fetch_log.toml";
