pub mod fetch_log;

use std::{fs, io, path::PathBuf};

use get_scryfall::{MtgoSet, ScryfallBulkData, ScryfallBulkDataInfo, ScryfallMtgoSets};

use crate::fetch_log::CardInfoMetaData;

const FETCH_LOG_FILENAME: &str = "fetch_log.toml";

/// Fetches all data if any needs updating and stores it in `save_to_dir`
///
pub fn fetch_all(save_to_dir: PathBuf) -> Result<(), io::Error> {
    let fetch_log_dst = save_to_dir.join(FETCH_LOG_FILENAME);
    let mut fetch_log: CardInfoMetaData = match fetch_log_dst.exists() {
        true => CardInfoMetaData::from_toml_file(&fetch_log_dst)?,
        false => CardInfoMetaData::new(),
    };
    log::debug!("fetch log contents: {fetch_log:?}");

    if fetch_log.is_next_set_out() {
        log::info!("Fetching sets from Scryfall");
        let sets = ScryfallMtgoSets::get().unwrap();
        let next_set: &MtgoSet = sets.next_released_mtgo_set().unwrap().unwrap();
        log::info!("Next released set is {next_set:?}");
        fetch_log.replace_next_released_set(next_set.into());
        let dst = save_to_dir.join(ScryfallMtgoSets::FILENAME);
        log::info!("Writing scryfall sets to {dst:?}");
        let sets_vec = sets.take_sets();
        fs::write(dst, serde_json::to_string(&sets_vec).unwrap())?;
    } else {
        log::info!("Scryfall sets data is up to date - skipping download");
    }

    let scryfall_bulk_info: ScryfallBulkDataInfo = ScryfallBulkDataInfo::get().unwrap();
    log::info!("scryfall bulk info: {scryfall_bulk_info:?}");
    if fetch_log.is_scryfall_bulk_updated(scryfall_bulk_info.updated_at()) {
        log::info!("Scryfall bulk data is up to date - skipping download")
    } else {
        log::info!("Fetching scryfall bulk data");
        let scryfall_bulk_data = ScryfallBulkData::get(scryfall_bulk_info.updated_at()).unwrap();
        fetch_log.refresh_bulk_data_updated_at_timestamp();
        let cards = scryfall_bulk_data.take_cards();
        let dst = save_to_dir.join(ScryfallBulkData::FILENAME);
        log::info!("Writing scryfall bulk data (default cards) to {dst:?}");
        fs::write(dst, serde_json::to_string(&cards).unwrap())?;
    }

    if fetch_log.is_card_definitions_updated() {
        log::info!("Card definitions are up to date - skipping download");
    } else {
        log::info!("Fetching Card definitions");
        let gb_card_defs = get_goatbots::get_goatbots_card_definitions();
        let dst = save_to_dir.join("card-definitions.json");
        log::info!("Writing Card definitions to {dst:?}");
        fs::write(dst, gb_card_defs)?;
        log::info!("Refreshing timestamp for fetching Goatbots card definitions");
        fetch_log.refresh_card_definitions_updated_at_timestamp();
    }

    // Get price history
    if fetch_log.is_goatbots_prices_updated() {
        log::info!("Prices are up to date - skipping download");
    } else {
        log::info!("Fetching Goatbots price history");
        let gb_price_hist = get_goatbots::get_goatbots_price_history();
        let dst = save_to_dir.join("price-history.json");
        log::info!("Writing Goatbots price history {dst:?}");
        fs::write(dst, gb_price_hist)?;
        log::info!("Refreshing timestamp for fetching Goatbots price history");
        fetch_log.refresh_prices_updated_at_timestamp();
    }

    // Save the log to disk
    fetch_log.to_toml_on_disk(&save_to_dir.join(CardInfoMetaData::FILENAME))
}

#[cfg(test)]
mod tests {
    use testresult::TestResult;

    use super::*;

    #[ignore = "Will download A LOT of data and save it to disk"]
    #[test]
    fn test_fetch_all() -> TestResult {
        let mut p = PathBuf::new();
        p.push("test");
        assert!(p.exists());

        get_scryfall::util::init_debug_logging(3);

        assert!(fetch_all(p).is_ok());
        Ok(())
    }
}
