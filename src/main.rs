extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use mtgo_collection_manager::download;
use serde::{Deserialize, Serialize};
use std::{collections, env, fs, io};

// TODO:
// Make stats file for collection, update on every run

#[derive(Debug, Serialize, Deserialize)]
struct NamePriceQuantity(String, f32, u32);

fn main() -> io::Result<()> {
    pretty_env_logger::init();
    info!("Starting price list manager!");
    // Create all directories if they don't exist
    let managed_dir_path = env::current_dir()?.join(mtgo_collection_manager::MANAGED_DIR);
    let prices_dir = managed_dir_path.join("prices");
    let collection_price_history_dir = managed_dir_path.join("collection-price-history");
    fs::create_dir_all(&managed_dir_path)?;
    fs::create_dir_all(&prices_dir)?;
    fs::create_dir_all(&collection_price_history_dir)?;

    let price_list = match download::lib::first_file_match_from_dir(
        mtgo_collection_manager::PRICE_LIST_FNAME,
        &prices_dir,
        Some(86400), // 24 hours
    ) {
        // Download and store price list if there's no file in the managed directory
        None => {
            let resp =
                download::lib::get_bytes_readable(mtgo_collection_manager::DOWNLOAD_PRICE_LIST_URL);
            let bytes = match resp {
                Ok(bytes) => bytes,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to download price list: {e}"),
                    ))
                }
            };

            let contents = download::lib::unzip_bytes(bytes)?;
            download::lib::store_contents(
                contents.clone(),
                mtgo_collection_manager::PRICE_LIST_FNAME,
                mtgo_collection_manager::MANAGED_PRICE_HISTORY,
            )?;
            contents
        }
        Some(price_list) => fs::read_to_string(price_list)?,
    };

    let card_definitions = match download::lib::first_file_match_from_dir(
        mtgo_collection_manager::CARD_DEFINITIONS_FNAME,
        &managed_dir_path,
        Some(86400), // 24 hours
    ) {
        // Download and store card definitions if there's no file in the managed directory
        None => {
            let response = download::lib::get_bytes_readable(
                mtgo_collection_manager::DOWNLOAD_CARD_DEFINITIONS_URL,
            );

            let bytes = match response {
                Ok(bytes) => bytes,
                Err(e) => {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Failed to download card definitions: {e}"),
                    ))
                }
            };

            let contents = download::lib::unzip_bytes(bytes)?;
            download::lib::store_contents(
                contents.clone(),
                mtgo_collection_manager::CARD_DEFINITIONS_FNAME,
                mtgo_collection_manager::MANAGED_DIR,
            )?;
            contents
        }
        Some(card_defs) => fs::read_to_string(card_defs)?,
    };

    let collection_map: collections::HashMap<String, (String, u32)> = if let Some(collection) =
        download::lib::first_file_match_from_dir("collection.json", &managed_dir_path, None)
    {
        let collection = fs::read_to_string(collection)?;
        serde_json::from_str(&collection)?
    } else {
        log::info!(
            "No collection.json found in managed directory, looking for Full Trade List instead"
        );
        let collection_map = if let Some(full_trade_list) =
            download::lib::first_file_match_from_dir("Full Trade List", &managed_dir_path, None)
        {
            let collection = fs::read_to_string(full_trade_list)?;
            let collection_map = mtgo_collection_manager::collection_map_from_string(collection);
            // Write to file
            mtgo_collection_manager::map_to_json_file(
                managed_dir_path,
                "collection.json",
                &collection_map,
            )?;
            collection_map
        } else {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to find Full Trade List in managed directory"),
            ));
        };
        collection_map
    };

    let prices_json: serde_json::Value =
        serde_json::from_str(&price_list).expect("JSON was not well-formatted");

    // No use for this yet
    let _card_defs_json: serde_json::Value =
        serde_json::from_str(&card_definitions).expect("JSON was not well-formatted");

    let collection_price_history_map: collections::HashMap<String, NamePriceQuantity> =
        match download::lib::first_file_match_from_dir(
            "collection-price-history-json",
            &collection_price_history_dir,
            Some(86400),
        ) {
            None => {
                // Create new collection price history map from collection map and price list
                let mut collection_price_history: collections::HashMap<String, NamePriceQuantity> =
                    collections::HashMap::new();
                for (id, (name, quantity)) in collection_map {
                    if id == "1" {
                        // This is an event ticket, insert with price 1.
                        collection_price_history
                            .insert(id, NamePriceQuantity(name.to_string(), 1.0, quantity));
                        continue;
                    }
                    let price = prices_json[id.as_str()].as_f64().expect(&format!(
                        "ID {id} did not yield a number, got: {:#?}",
                        prices_json[id.as_str()]
                    ));
                    collection_price_history.insert(
                        id,
                        NamePriceQuantity(name.to_string(), price as f32, quantity),
                    );
                }
                // Store it
                let collection_price_history_json =
                    serde_json::to_string(&collection_price_history)?;
                download::lib::store_contents(
                    collection_price_history_json,
                    "collection-price-history-json",
                    "managed-files\\collection-price-history\\",
                )?;
                collection_price_history
            }
            Some(collection_price_history) => {
                let collection_price_history = fs::read_to_string(collection_price_history)?;
                serde_json::from_str(&collection_price_history)?
            }
        };

    let total_quantity: u32 = collection_price_history_map
        .values()
        .map(|NamePriceQuantity(_, _, quantity)| quantity)
        .sum();

    println!(
        "{} unique items in collection",
        collection_price_history_map.len()
    );
    println!("{total_quantity} total quantity");

    // top 10 most expensive cards
    let mut top_10: Vec<NamePriceQuantity> = collection_price_history_map
        .values()
        .map(|NamePriceQuantity(name, price, quantity)| {
            NamePriceQuantity(name.to_string(), *price, *quantity)
        })
        .collect();
    top_10.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("Top 10 most expensive cards:");
    top_10
        .iter()
        .take(10)
        .for_each(|NamePriceQuantity(name, price, quantity)| {
            println!("{name}: {price}$ - {quantity} pcs.")
        });
    // Total value
    let total_value: f32 = collection_price_history_map
        .values()
        .map(|NamePriceQuantity(_, price, quantity)| price * (*quantity as f32))
        .sum();
    println!("Total value (with Goatbots sell price): {total_value} $");

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json;

    #[test]
    #[ignore] // Ignore because this actually downloads the file from goatbots.com
    fn test_get_zip_and_unzip() {
        let price_url = "https://www.goatbots.com/download/price-history.zip";

        let res_bytes = reqwest::blocking::get(price_url).unwrap().bytes().unwrap();

        println!("Response bytes length: {:?}", res_bytes.len());

        let mut archive = zip::ZipArchive::new(std::io::Cursor::new(res_bytes)).unwrap();
        let mut file = archive.by_index(0).unwrap();
        let mut contents = String::new();
        std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
        // for line in contents.lines() {
        //     println!("{}", line);
        // }
    }

    #[test]
    fn test_card_defs() {
        let card_definitions = std::fs::read_to_string("card-definitions.txt").unwrap();
        let prices = std::fs::read_to_string("prices\\price-history-2023-04-03T21-10.txt").unwrap();
        let prices_json: serde_json::Value =
            serde_json::from_str(prices.as_str()).expect("JSON was not well-formatted");
        let cards_json: serde_json::Value =
            serde_json::from_str(card_definitions.as_str()).expect("JSON was not well-formatted");
        let card = cards_json["108818"].as_object().unwrap();
        let price = prices_json["108818"].as_f64().unwrap();
        println!("Card definitions: {:?}", card);
        println!(
            "Name: {}\nPrice: {:?}",
            card["name"].as_str().unwrap(),
            price
        );
        let keys_cards = cards_json.as_object().unwrap().keys();
        keys_cards.into_iter().for_each(|key| {
            let card = cards_json[key].as_object().unwrap();
            let price = prices_json[key].as_f64().unwrap();
            println!(
                "Name: {}\nPrice: {:?}",
                card["name"].as_str().unwrap(),
                price
            );
        });
    }

    #[test]
    fn test_time_str() {
        let mut path = std::env::current_dir().unwrap();
        println!("The current directory is {}", path.display());
        path.push("prices\\");
        println!("Path is {}", path.display());
        let dt: chrono::DateTime<chrono::Local> = chrono::Local::now();
        let time_str = dt.format("%Y-%m-%dT%H-%M").to_string();
        println!("Time str is {}", time_str);
        let filename = format!("price-history-{time_str}.txt");
        println!("Filename is {}", filename);
        path.push(filename.to_string());

        use std::fs::OpenOptions;

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .unwrap();
        use std::io::prelude::*;
        // Write a &str in the file (ignoring the result).
        writeln!(&mut file, "Hello World!").unwrap();

        file.flush().unwrap();
    }

    #[test]
    fn test_xml_parse() {
        let mut path = dirs::download_dir().unwrap();
        path.push("Full Trade List.dek");
        let file = std::fs::File::open(path).unwrap();
        let mut file = std::io::BufReader::new(file);

        let mut buf = String::new();
        let _bytes_read = file.read_to_string(&mut buf).unwrap();
        let pattern =
            r#"CatID="(?P<id>[0-9]*).*Quantity="(?P<quantity>[0-9]*).*Name="(?P<name>[\s\w]*)"#;
        let re: regex::Regex = regex::Regex::new(pattern).unwrap();
        let collection_map: std::collections::HashMap<String, (String, u32)> = re
            .captures_iter(buf.as_str())
            .map(|cap| {
                (
                    cap.name("id").unwrap().as_str().to_string(),
                    (
                        cap.name("name").unwrap().as_str().to_string(),
                        cap.name("quantity")
                            .unwrap()
                            .as_str()
                            .parse::<u32>()
                            .unwrap(),
                    ),
                )
            })
            .collect();

        // Write to file
        let mut path = std::env::current_dir().unwrap();
        path.push("collection.json");
        let mut output_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(path)
            .unwrap();
        use std::io::prelude::*;
        writeln!(
            output_file,
            "{}",
            serde_json::to_string_pretty(&collection_map).unwrap()
        )
        .unwrap();
    }
}
