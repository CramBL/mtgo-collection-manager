use std::{collections::hash_map::HashMap, path::Path};

pub fn parse_price_history_json(
    price_history_json: &Path,
) -> Result<HashMap<String, f32>, Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string(price_history_json)?;
    let price_history: HashMap<String, f32> = serde_json::from_str(&json)?;
    Ok(price_history)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use testresult::TestResult;

    #[test]
    fn test_price_history_5_cards() -> TestResult {
        let path = Path::new(r"../test/test-data/goatbots/price-hist-small-5cards.json");
        let price_history = parse_price_history_json(path)?;
        assert_eq!(price_history.len(), 5);
        assert_eq!(price_history.get("112348").unwrap(), &0.003);
        assert_eq!(price_history.get("40516").unwrap(), &1.03);
        assert_eq!(price_history.get("31745").unwrap(), &0.37);
        assert_eq!(price_history.get("348").unwrap(), &419.99);
        assert_eq!(price_history.get("347").unwrap(), &244.99);
        Ok(())
    }

    #[test]
    fn test_price_history_full() -> TestResult {
        let path = Path::new(r"../test/test-data/goatbots/price-history-2023-10-02-full.json");
        let price_history = parse_price_history_json(path)?;
        assert_eq!(price_history.len(), 76070);
        assert_eq!(price_history.get("112348").unwrap(), &0.003);

        Ok(())
    }
}
