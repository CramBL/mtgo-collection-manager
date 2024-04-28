use std::{fs, io, path::Path};

use chrono::Utc;
use collection::Collection;
use goatbots::{card_definitions::parse_card_def_json, price_history::parse_price_history_json};
use mtgo_card::MtgoCard;
use scryfall::default_cards::ScryfallCard;
use xml::parse_dek_xml;

pub mod collection;
pub mod goatbots;
pub mod mtgo_card;
pub mod scryfall;
pub mod util;
pub mod xml;

pub fn parse_full(
    full_trade_list_path: &Path,
    scryfall_path: &Path,
    card_definitions_path: &Path,
    price_history_path: &Path,
    save_json_to_dir: Option<&Path>,
) -> Result<Vec<MtgoCard>, io::Error> {
    let xml_cards =
        parse_dek_xml(full_trade_list_path).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let price_hist = parse_price_history_json(price_history_path).unwrap();
    let goatbots_card_defs = parse_card_def_json(card_definitions_path).unwrap();
    let scryfall_json_str = fs::read_to_string(scryfall_path)?;
    let scryfall_cards: Vec<ScryfallCard> = serde_json::from_str(&scryfall_json_str).unwrap();
    let mut collection = Collection::from_xml_cards(xml_cards).unwrap();
    collection
        .extract_goatbots_info(goatbots_card_defs, price_hist)
        .unwrap();
    collection.extract_scryfall_info(scryfall_cards).unwrap();

    if let Some(p) = save_json_to_dir {
        if has_state_log_changed(p) {
            let fname = "state_log.toml";
            let state_log_path = p.join(fname);
            let hist_log_dir = p.join("collection-history");
            if !hist_log_dir.exists() {
                fs::create_dir_all(&hist_log_dir).unwrap();
            }
            let hist_log_path = hist_log_dir.join(fname);
            fs::copy(state_log_path, hist_log_path).unwrap();
        }
        // Save json
        let json_out = serde_json::to_string(&collection)?;
        let timestamp = Utc::now();
        let fname = format!("mtgo-cards_{}", timestamp.format("%Y-%m-%dT%H%M%SZ"));
        fs::write(p.join(fname), json_out).unwrap();
    }
    Ok(collection.take_cards())
}

pub fn has_state_log_changed(appdata_dir: &Path) -> bool {
    let fname = "state_log.toml";
    let history_log_path = appdata_dir.join("collection-history").join(fname);
    let appdata_log_path = appdata_dir.join(fname);
    if history_log_path.exists() {
        let app_log_md = fs::metadata(&appdata_log_path).unwrap();
        let hist_log_md = fs::metadata(history_log_path).unwrap();
        if app_log_md.len() != hist_log_md.len() {
            return true;
        } else {
            let app_log_contents = fs::read(&appdata_log_path).unwrap();

            let hist_log_contents = fs::read(&appdata_log_path).unwrap();
            return app_log_contents != hist_log_contents;
        }
    } else {
        true
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_str_eq;

    #[test]
    pub fn test_timestamped_filename() {
        let timestamp = Utc::now();
        let ts_str = format!("{}", timestamp.format("%Y-%m-%dT%H%M%SZ"));
        let fname = format!("mtgo-cards_{ts_str}");
        eprintln!("{fname}");
    }
}
