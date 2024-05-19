use std::{
    fs, io,
    path::{Path, PathBuf},
};

use chrono::{DateTime, NaiveDateTime, Utc};

/// Get all files in a directory that have a timestamp suffix of pattern `YYYY-MM-DDThhmmssZ`.
pub fn get_files_with_timestamp(dir: &Path) -> Result<Vec<(PathBuf, DateTime<Utc>)>, io::Error> {
    let mut files: Vec<(PathBuf, DateTime<Utc>)> = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if name.ends_with('Z') {
            // Index where the timestamp of pattern `YYYY-MM-DDThhmmssZ` starts
            let start_of_timestamp = name.len() - 18;
            // Will fail in about 1000 years.
            debug_assert_eq!(name.chars().nth(start_of_timestamp), Some('2'), "Expected char '2' at position {start_of_timestamp}, it should be the beginning of a timestamp of pattern YYYY-MM-DDThhmmssZ and therefor indicate the millenia 2 thousand");

            let timestamp = &name[start_of_timestamp..];
            let timestamp = NaiveDateTime::parse_from_str(timestamp, "%Y-%m-%dT%H%M%SZ")
                .unwrap()
                .and_utc();
            files.push((entry.path(), timestamp));
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use temp_dir::TempDir;
    use testresult::TestResult;

    #[test]
    fn test_get_files_with_timestamp() -> TestResult {
        let temp_dir = TempDir::new()?;
        let tmp_file0 = temp_dir.path().join("mtgo_cards_2023-11-05T152700Z");
        let tmp_file1 = temp_dir.path().join("mtgo_cards_2023-11-05T152800Z");

        fs::write(&tmp_file0, "content")?;
        fs::write(&tmp_file1, "content")?;

        let files = get_files_with_timestamp(temp_dir.path())?;

        assert_eq!(files.len(), 2);

        let (file0, timestamp0) = &files[0];
        let (file1, timestamp1) = &files[1];

        // 1 and 0 are swapped because they are read in reverse order relative to when they were written
        assert_eq!(file0, &tmp_file1);
        assert_eq!(file1, &tmp_file0);

        assert_eq!(
            timestamp0.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "2023-11-05T15:28:00Z"
        );
        assert_eq!(
            timestamp1.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
            "2023-11-05T15:27:00Z"
        );

        Ok(())
    }
}
