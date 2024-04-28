use std::ffi::OsStr;

use fltk::{
    app::{self, Sender},
    enums::Event,
    prelude::{TableExt, WidgetBase},
};
use fltk_table::SmartTable;
use mtgoupdater::MtgoCard;

use crate::{
    collection::view::table::{SortStates, SortedBy},
    Message,
};

use super::{
    column::{Column, Ordering},
    CollectionTable,
};

/// Basic column layout for the collection table
pub struct CollectionColumn {
    /// The name of the column
    pub name: &'static str,
    /// The width of the column
    pub width: i32,
    /// The index of the column
    pub idx: i32,
}

impl CollectionColumn {
    /// Declare a new column with an associated index, name, and width
    ///
    /// # Arguments
    ///
    /// * `idx` - The index of the column
    /// * `name` - The name of the column
    /// * `width` - The width of the column
    ///
    /// # Returns
    ///
    /// A new [CollectionColumn] instance
    pub const fn new(idx: i32, name: &'static str, width: i32) -> Self {
        Self { idx, name, width }
    }

    /// Fill the cell at the given row with the given value
    ///
    /// # Arguments
    ///
    /// * `table` - The [SmartTable] to fill
    /// * `row_idx` - The index of the row to fill
    /// * `val` - The value to fill the cell with
    pub fn fill(&self, table: &mut SmartTable, row_idx: i32, val: &str) {
        table.set_cell_value(row_idx, self.idx, val);
    }
}

/// Sort the rows (card) of the table by the given column
pub fn sort_cards(cards: &mut [MtgoCard], sort_states: &mut SortStates, category: Column) {
    match category {
        Column::Name => {
            if sort_states.name_ord().is_descending() {
                cards.sort_by(|a, b| b.name.cmp(&a.name));
                sort_states.set_name_ord(SortedBy::Name(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| a.name.cmp(&b.name));
                sort_states.set_name_ord(SortedBy::Name(Ordering::Descending));
            }
        }
        Column::Quantity => {
            if sort_states.quantity_ord().is_descending() {
                cards.sort_by(|a, b| a.quantity.cmp(&b.quantity));
                sort_states.set_quantity_ord(SortedBy::Quantity(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| b.quantity.cmp(&a.quantity));
                sort_states.set_quantity_ord(SortedBy::Quantity(Ordering::Descending));
            }
        }
        Column::Foil => {
            if sort_states.foil_ord().is_descending() {
                cards.sort_by(|a, b| a.foil.cmp(&b.foil));
                sort_states.set_foil_ord(SortedBy::Foil(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| b.foil.cmp(&a.foil));
                sort_states.set_foil_ord(SortedBy::Foil(Ordering::Descending));
            }
        }
        Column::Goatbots => {
            if sort_states.goatbots_ord().is_descending() {
                cards.sort_by(|a, b| a.goatbots_price.partial_cmp(&b.goatbots_price).unwrap());
                sort_states.set_goatbots_ord(SortedBy::Goatbots(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| b.goatbots_price.partial_cmp(&a.goatbots_price).unwrap());
                sort_states.set_goatbots_ord(SortedBy::Goatbots(Ordering::Descending));
            }
        }
        Column::Scryfall => {
            if sort_states.cardhoarder_ord().is_descending() {
                cards.sort_by(|a, b| a.scryfall_price.partial_cmp(&b.scryfall_price).unwrap());
                sort_states.set_cardhoarder_ord(SortedBy::Scryfall(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| b.scryfall_price.partial_cmp(&a.scryfall_price).unwrap());
                sort_states.set_cardhoarder_ord(SortedBy::Scryfall(Ordering::Descending));
            }
        }
        Column::Set => {
            if sort_states.set_ord().is_descending() {
                cards.sort_by(|a, b| b.set.cmp(&a.set));
                sort_states.set_set_ord(SortedBy::Set(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| a.set.cmp(&b.set));
                sort_states.set_set_ord(SortedBy::Set(Ordering::Descending));
            }
        }
        Column::Rarity => {
            if sort_states.rarity_ord().is_descending() {
                cards.sort_by(|a, b| a.rarity.cmp(&b.rarity));
                sort_states.set_rarity_ord(SortedBy::Rarity(Ordering::Ascending));
            } else {
                cards.sort_by(|a, b| b.rarity.cmp(&a.rarity));
                sort_states.set_rarity_ord(SortedBy::Rarity(Ordering::Descending));
            }
        }
    }
}

/// Drag and drop a file onto the table invokes this callback
///
/// Takes the path to the file and forwards it with the event: [Message::GotFullTradeList]
/// If the path is not a common filepath, an attempt is made to parse it as an URI to a filepath.
///
/// # Arguments
///
/// * `table` - The [SmartTable] to set the callback on
/// * `ev_sender` - The [Sender] to send the [Message] to
///
/// # Example
///
/// ```
/// use fltk::{app, prelude::*, table::TableExt};
/// use fltk_table::SmartTable;
/// use mtgogui::collection::view::table::util::set_drag_and_drop_callback;
///
/// let app = app::App::default();
///
/// let mut table = SmartTable::new(0, 0, 0, 0, "");
/// set_drag_and_drop_callback(&mut table, app::channel().0);
/// ```
pub fn set_drag_and_drop_callback(table: &mut SmartTable, ev_sender: Sender<Message>) {
    table.handle({
        let mut dnd = false;
        let mut released = false;
        move |_, ev| match ev {
            Event::DndEnter => {
                dnd = true;
                true
            }
            Event::DndDrag => true,
            Event::DndRelease => {
                released = true;
                true
            }
            Event::Paste => {
                if dnd && released {
                    let path_str = app::event_text();
                    log::info!("path: {path_str}");
                    let path = std::path::PathBuf::from(&path_str);
                    match path.try_exists() {
                        Ok(path_exists) => {
                            if path_exists {
                                // Path exists, ship it.
                                ev_sender.send(Message::GotFullTradeList(path.into()));
                            } else {
                                // Doesn't exist? Try to parse it to a file path
                                match url::Url::parse(&path_str) {
                                    Ok(path_url) => // Extract the path component from the URI
                                    if let Ok(path_buf) = path_url.to_file_path() {
                                        match path_buf.try_exists() {
                                            Ok(path_exists) => {
                                                if path_exists {
                                                    log::info!("All good after URL parsing");
                                                    // Ship it
                                                    ev_sender.send(Message::GotFullTradeList(
                                                        path_buf.into(),
                                                    ));
                                                } else {
                                                    log::info!("URL parsing succeeded, but the path doesn't exist.");
                                                }
                                            }
                                            Err(e) => log::error!("Checking if path exists failed: {e}"),
                                        }
                                    } else {
                                        log::error!("Failed to parse URI to path: {path_url}");
                                    },
                                    Err(e) => log::info!("Failed to parse URI from drag and drop: {e}"),
                                }
                            }
                        }
                        Err(e) => log::error!("Checking if path exists failed: {e}"),
                    }

                    dnd = false;
                    released = false;
                    true
                } else {
                    false
                }
            }
            Event::DndLeave => {
                dnd = false;
                released = false;
                true
            }
            _ => false,
        }
    });
}

/// Iterates over the [SmartTable] and the [MtgoCard]s filling out all the cells of the table.
/// If the amount of rows in the table is less than the amount of cards, the table is extended.
///
/// # Arguments
///
/// * `table` - The [SmartTable] to fill
/// * `cards` - A borrowed slice with the [MtgoCard]s to fill the table with
pub fn draw_cards(table: &mut SmartTable, cards: &[MtgoCard]) {
    if cards.is_empty() {
        return;
    }

    // Extend the table with rows matching the amount of cards
    if cards.len() > table.row_count() as usize {
        let cards_minus_rows = cards.len() - table.row_count() as usize;
        for _ in 0..(cards_minus_rows) {
            table.append_empty_row("");
        }
    }

    // Iterate over the rows and filling each column of a row with values from cards
    for (idx, card) in cards.iter().enumerate() {
        let row_idx = idx as i32;
        fill_card_row(table, row_idx, card);
    }
}

/// Helper to fill a single row with [MtgoCard] data
///
/// # Arguments
///
/// * `table` - The [SmartTable] to fill
/// * `row_idx` - The index of the row to fill
/// * `card` - The [MtgoCard] to fill the row with
pub fn fill_card_row(table: &mut SmartTable, row_idx: i32, card: &MtgoCard) {
    CollectionTable::COL_NAME.fill(table, row_idx, &card.name);
    CollectionTable::COL_QUANTITY.fill(table, row_idx, &card.quantity.to_string());
    CollectionTable::COL_FOIL.fill(table, row_idx, if card.foil { "Yes" } else { "No" });
    CollectionTable::COL_GOATBOTS.fill(table, row_idx, &format!("{:8.3}", card.goatbots_price));
    CollectionTable::COL_CARDHOARDER.fill(table, row_idx, &{
        if let Some(p) = card.scryfall_price {
            format!("{p:8.3}")
        } else {
            "N/A".into()
        }
    });
    CollectionTable::COL_SET.fill(table, row_idx, &card.set);
    CollectionTable::COL_RARITY.fill(table, row_idx, &card.rarity.to_string());
}
