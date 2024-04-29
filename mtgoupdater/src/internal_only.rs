use crate::{set_mtgogetter_bin, MTGOGETTER_BIN};

// Safe to call multiple times from different threads (for tests)
pub fn dev_try_init_mtgogetter_bin() {
    if MTGOGETTER_BIN.get().is_none() {
        _ = set_mtgogetter_bin(DEV_MTGOGETTER_BIN.into());
    }
}

// Path to the MTGO Getter binary in the repository
pub const DEV_MTGOGETTER_BIN: &str = if cfg!(windows) {
    "../mtgogetter/mtgogetter.exe"
} else {
    "../mtgogetter/mtgogetter"
};
