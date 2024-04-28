use std::io::{self};
use std::path::{Path, PathBuf};
use std::{env, fs};

// Convenience macro for printing warnings during the build process
//
// Currently the only way to print "info" messages during the build process (see: https://github.com/rust-lang/cargo/issues/985)
macro_rules! print_warn {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    util::detect_changes();

    if cfg!(debug_assertions) {
        print_warn!("Debug mode, skipping custom release build steps");
        return;
    }

    let env_out_dir = env::var_os("OUT_DIR").expect("OUT_DIR not set");
    let out_dir_path = Path::new(&env_out_dir);

    build::build_all(out_dir_path);

    include_binaries(out_dir_path)
        .unwrap_or_else(|e| panic!("Failed to include binaries in build.rs: {e}"));
}

/// Binary name for the MTGO Getter binary
const MTGOGETTER_BIN: &str = if cfg!(target_os = "windows") {
    "mtgogetter.exe"
} else {
    "mtgogetter"
};

/// Name of the file that contains the byte arrays for the binaries
const INCLUDE_BINARIES_FILE: &str = "include_binaries.rs";

mod util {
    use std::{
        fs::File,
        io::{self, Read},
        path::Path,
    };

    /// Rerun the build script steps if any of these files or directories change
    pub fn detect_changes() {
        // Build script itself
        println!("cargo:rerun-if-changed=build.rs");
        // MTGO Updater
        println!("cargo:rerun-if-changed=../mtgoupdater/src");
        // MTGO Getter
        println!("cargo:rerun-if-changed=../mtgogetter/main.go");
        println!("cargo:rerun-if-changed=../mtgogetter/pkg");
        println!("cargo:rerun-if-changed=../mtgogetter/cmd");
    }

    /// Get the length (size in bytes) of a file
    ///
    /// Reads the whole file into memory and returns the length of the vector.
    /// This is more reliable than using `std::fs::metadata` because it doesn't rely on the file system.
    pub fn file_len(fpath: &Path) -> io::Result<usize> {
        const PRE_ALLOC: usize = 1024 * 1024 * 20; // 20 MiB
        let mut file = File::open(fpath)?;
        let mut raw_mtgogetter = Vec::with_capacity(PRE_ALLOC);
        file.read_to_end(raw_mtgogetter.as_mut())?;
        Ok(raw_mtgogetter.len())
    }
}

/// Write the binaries to the OUT_DIR and create a file `include_binaries.rs` that contains the binaries as byte arrays.
fn include_binaries(out_dir: &Path) -> io::Result<()> {
    let dest_path = Path::new(&out_dir).join(INCLUDE_BINARIES_FILE);
    let mtgogetter_path = Path::new(&out_dir).join(MTGOGETTER_BIN);

    let mtgogetter_size = util::file_len(&mtgogetter_path)?;

    // format contents
    let contents = format_include_binaries_rs(mtgogetter_size, mtgogetter_path);

    // Write the file contents
    fs::write(dest_path, contents)?;

    Ok(())
}

/// Format the contents of the `include_binaries.rs` file
fn format_include_binaries_rs(mtgogetter_size: usize, mtgogetter_path: PathBuf) -> String {
    format!(
        r#"
        #[cfg(not(debug_assertions))]
        pub const MTGOGETTER: &[u8; {mtgogetter_size}] = include_bytes!({mtgogetter_path:?});
        {WRITE_TO_BIN_DIR_FN}
        "#
    )
}

/// Code to write the binaries to the bin directory
///
/// Creates the `bin` directory in the directory where the binary `MTGO GUI` is located.
///
/// Writes the binaries to the `bin` directory if they don't already exist.
const WRITE_TO_BIN_DIR_FN: &str = r#"
#[cfg(not(debug_assertions))]
fn write_binaries_out() -> std::io::Result<()> {
    const MTGOGETTER_BIN: &str = if cfg!(target_os = "windows") {
        "mtgogetter.exe"
    } else {
        "mtgogetter"
    };

    let mut path = std::env::current_exe()?;
    path.pop();
    path.push("bin");

    if !path.exists() {
        std::fs::create_dir(&path)?;
    }

    let mtgogetter_bin = path.join(MTGOGETTER_BIN);

    if !mtgogetter_bin.exists() {
        std::fs::write(&mtgogetter_bin, MTGOGETTER)?;
        if !cfg!(windows) {
            let exit_status = std::process::Command::new("chmod")
                    .arg("+x")
                    .arg(format!("{}", mtgogetter_bin.display()))
                    .status()
                    .expect("Failed execute chmod to make mtgogetter executable");

            assert!(
                exit_status.success(),
                "failed to make MTGO Getter executable"
            );
        }
    }

    Ok(())
}
"#;

mod build {
    use crate::MTGOGETTER_BIN;

    use std::{error::Error, fs, path::Path};

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    /// Build all binaries and copy them to the OUT_DIR
    pub fn build_all(out_dir: &Path) {
        // Build MTGO Getter
        build_mtgogetter(out_dir).unwrap_or_else(|e| {
            panic!("Failed to build MTGO Getter and copy to OUT_DIR: {out_dir:?}: {e}")
        })
    }

    /// Build MTGO Getter and copy the binary to the OUT_DIR set by cargo
    fn build_mtgogetter(out_dir: &Path) -> Result<()> {
        let mut cmd_build_mtgogetter = std::process::Command::new("task");

        let cmd_build_mtgogetter = cmd_build_mtgogetter
            .arg("mtgogetter:build")
            .current_dir("..")
            .status()?;

        assert!(
            cmd_build_mtgogetter.success(),
            "failed to build MTGO Getter"
        );

        // Copy mtgogetter to the OUT_DIR set by cargo
        let rel_mtgogetter_bin_str = format!("../mtgogetter/{MTGOGETTER_BIN}");
        let rel_mtgogetter_bin_path = Path::new(&rel_mtgogetter_bin_str);

        assert!(
            rel_mtgogetter_bin_path.exists(),
            "Build succeeded but mtgogetter was not found at the expected path ../mtgogetter/{MTGOGETTER_BIN}"
        );

        let dest_path = Path::new(&out_dir).join(MTGOGETTER_BIN);
        fs::copy(rel_mtgogetter_bin_path, &dest_path)?;

        print_warn!("Built MTGO Getter and copied the binary to {dest_path:?}");

        Ok(())
    }
}
