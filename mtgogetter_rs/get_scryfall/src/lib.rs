use serde::{Deserialize, Serialize};

pub mod bulk_cards;
pub mod bulk_info;
pub mod util;

#[cfg(test)]
mod tests {
    use testresult::TestResult;

    use super::*;

    #[ignore = "Will download data from the Scryfall API"]
    #[test]
    fn test_get_scryfall_bulk_info() -> TestResult {
        Ok(())
    }
}
