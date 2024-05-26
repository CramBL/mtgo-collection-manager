use std::vec::Drain;

use mtgogetter_rs::fetch_log::CardInfoMetaData;

use super::{
    container::CollectionStats,
    util::{CategoryStat, MultiValueStat, UniqueTotal},
};

pub struct BrowserItems {
    item_index: usize,
    formatted_items: Vec<String>,
    title_format: String,
    alt_title_format: String,
    value_format: String,
    alt_value_format: String,
}

impl BrowserItems {
    const ALTERNATING_BACKGROUND_COLOR: &'static str = r#"@B49"#; // Dark grey

    const TITLE_FONT_SIZE: &'static str = r#"@S15"#;
    const TITLE_FONT_BOLD: &'static str = r#"@b"#;

    const VALUE_FONT_SIZE: &'static str = r#"@S13"#;
    const VALUE_FONT_TYPE: &'static str = r#"@F13"#;

    fn title_format(&self) -> &str {
        if self.item_index % 2 == 0 {
            &self.title_format
        } else {
            &self.alt_title_format
        }
    }

    fn value_format(&self) -> &str {
        if self.item_index % 2 == 0 {
            &self.value_format
        } else {
            &self.alt_value_format
        }
    }

    pub fn new() -> Self {
        Self {
            item_index: 0,
            formatted_items: Vec::new(),
            title_format: format!(
                "{font_size}{bold}@.",
                bold = Self::TITLE_FONT_BOLD,
                font_size = Self::TITLE_FONT_SIZE
            ),
            alt_title_format: format!(
                "{font_size}{bold}{background_color}@.",
                bold = Self::TITLE_FONT_BOLD,
                font_size = Self::TITLE_FONT_SIZE,
                background_color = Self::ALTERNATING_BACKGROUND_COLOR
            ),
            value_format: format!(
                "{font_size}{font_type}",
                font_size = Self::VALUE_FONT_SIZE,
                font_type = Self::VALUE_FONT_TYPE
            ),
            alt_value_format: format!(
                "{font_size}{font_type}{background_color}",
                font_size = Self::VALUE_FONT_SIZE,
                font_type = Self::VALUE_FONT_TYPE,
                background_color = Self::ALTERNATING_BACKGROUND_COLOR
            ),
        }
    }

    /// Append the contents of `other` to `self`
    pub fn append(&mut self, other: &mut Self) {
        self.formatted_items.append(&mut other.formatted_items);
    }

    pub fn add_item(&mut self, title: &str, value: &str) {
        let formatted_item = format!(
            "{title_format}{title}\t{value_format}@.{value}",
            title_format = self.title_format(),
            value_format = self.value_format(),
            title = title,
            value = value
        );
        self.formatted_items.push(formatted_item);
        self.item_index += 1;
    }

    // TODO: Add good way to supply format strings for the arguments
    pub fn add_item_unique_total(&mut self, title: &str, unique_total_pair: UniqueTotal) {
        let formatted_item = format!(
            "{title_format}{title}\t{value_format}@.{unique} ({total})",
            title_format = self.title_format(),
            value_format = self.value_format(),
            title = title,
            unique = unique_total_pair.unique(),
            total = unique_total_pair.total()
        );
        self.formatted_items.push(formatted_item);
        self.item_index += 1;
    }

    // TODO: Add good way to supply format strings for the arguments
    pub fn add_multi_value_item(&mut self, mut stat: MultiValueStat) {
        let mut first_item = format!(
            "{title_format}{title}\t{value_format}",
            title_format = self.title_format(),
            value_format = self.value_format(),
            title = stat.title()
        );

        let values = stat.take_values();
        let mut value_iter = values.iter();
        if let Some(first_value) = value_iter.next() {
            first_item.push_str(first_value);
        }
        self.formatted_items.push(first_item);
        // Now add any remaining values
        for value in value_iter {
            let formatted_item = format!(
                "{title_format} \t{value_format}{value}",
                title_format = self.title_format(),
                value_format = self.value_format(),
                value = value
            );
            self.formatted_items.push(formatted_item);
        }
        self.item_index += 1;
    }

    pub fn add_category_item(&mut self, mut cat: CategoryStat) {
        let cat_title = format!(
            "@_{title_format}{title}\t{value_format}",
            title_format = self.title_format(),
            value_format = self.value_format(),
            title = cat.title()
        );

        self.formatted_items.push(cat_title);

        for (description, value) in cat.take_value_pairs() {
            let formatted_item = format!(
                //@S13@c@.
                "@S13@r@.{description}   \t{value_format}@.{value}",
                value_format = self.value_format(),
            );
            self.formatted_items.push(formatted_item);
        }
        self.item_index += 1;
    }

    pub fn drain(&mut self) -> Drain<'_, String> {
        self.formatted_items.drain(..)
    }
}

/// Converts a [CollectionStats] into a [BrowserItems] that can be displayed in a [Browser](fltk::browser::Browser)
///
/// # Errors
///
/// Returns an error if any of the stats in [CollectionStats] are missing
impl TryFrom<CollectionStats> for BrowserItems {
    type Error = String;

    fn try_from(mut stats: CollectionStats) -> Result<Self, Self::Error> {
        let mut browser_items = BrowserItems::new();
        browser_items.add_item_unique_total("Total items", stats.total_cards());

        if let Some(tot_stat_val) = stats.take_total_value() {
            browser_items.add_multi_value_item(tot_stat_val);
        } else {
            return Err("No total value stat set".into());
        }

        if let Some(most_expensive_item_stat_val) = stats.take_most_expensive_item() {
            browser_items.add_multi_value_item(most_expensive_item_stat_val);
        } else {
            return Err("No most expensive item stat set".into());
        }
        browser_items.add_item_unique_total("Cards > 5 tix", stats.cards_over_5_tix());
        browser_items.add_item_unique_total("Cards < 0.1 tix", stats.cards_under_a_tenth_tix());
        if let Some(rarity_dist_stat_val) = stats.take_rarity_distribution() {
            browser_items.add_multi_value_item(rarity_dist_stat_val);
        } else {
            return Err("No rarity distribution stat set".into());
        }
        Ok(browser_items)
    }
}

impl TryFrom<CardInfoMetaData> for BrowserItems {
    type Error = String;

    fn try_from(value: CardInfoMetaData) -> Result<Self, Self::Error> {
        log::info!("Converting metadata to browser items");
        let mut items = BrowserItems::new();

        let last_updated = CategoryStat::new(
            "Prices updated".into(),
            vec![
                (
                    "Goatbots".into(),
                    value
                        .goatbots_prices_updated_at()
                        .map_or_else(|| "N/A".to_string(), |d| d.to_string()),
                ),
                (
                    "Cardhoarder".into(),
                    value
                        .scryfall_bulk_data_updated_at()
                        .map_or_else(|| "N/A".to_string(), |d| d.to_string()),
                ),
            ],
        );
        log::info!("Adding last updated category item");
        items.add_category_item(last_updated);

        Ok(items)
    }
}
