#![allow(dead_code)]

use chrono::NaiveDate;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct NextReleasedMtgoSet {
    name: Option<String>,
    released_at: Option<NaiveDate>,
    mtgo_code: Option<String>,
}

impl NextReleasedMtgoSet {
    pub fn new(
        name: Option<String>,
        released_at: Option<NaiveDate>,
        mtgo_code: Option<String>,
    ) -> Self {
        Self {
            name,
            released_at,
            mtgo_code,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn released_at(&self) -> Option<NaiveDate> {
        self.released_at
    }

    pub fn mtgo_code(&self) -> Option<&str> {
        self.mtgo_code.as_deref()
    }

    /// Returns true if any of the fields are [None]
    ///
    /// # Note
    /// This is a precaution.
    /// It is expected that if one is none then all will be, which means a fetch of the newest data is required.
    ///
    /// However, if such a case could arise where some but not all fields are [None]
    /// then it is safe to assume that the data needs to be refreshed similarly to the expected case.
    ///
    /// For these reasons, this function is provided in place of one that only checks if all fields are [None].
    pub fn is_any_none(&self) -> bool {
        self.name().is_none() || self.released_at().is_none() || self.mtgo_code().is_none()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};
    use testresult::TestResult;

    #[test]
    fn test_next_released_mtgoset() -> TestResult {
        let naive_date = NaiveDate::from_ymd_opt(2023, 12, 11).unwrap();
        let next_released_mtgo_set = NextReleasedMtgoSet {
            name: Some("The Lost Caverns of Ixalan".to_string()),
            released_at: Some(naive_date),
            mtgo_code: Some("lci".to_string()),
        };

        let toml_str = toml::to_string(&next_released_mtgo_set)?;
        assert_str_eq!(
            toml_str,
            "\
            name = \"The Lost Caverns of Ixalan\"\n\
            released_at = \"2023-12-11\"\n\
            mtgo_code = \"lci\"\n\
            ",
        );
        let from_toml_str = toml::from_str(&toml_str)?;
        assert_eq!(next_released_mtgo_set, from_toml_str);

        Ok(())
    }
}
