use crate::assets::{get_asc_svg, get_desc_svg};
use crate::Message;
use fltk::{app, button, group::Column};
use fltk::{
    app::App,
    enums::{Align, Color, Event, FrameType},
    image,
    prelude::*,
    prelude::{GroupExt, TableExt, WidgetExt},
};
use fltk_table::{SmartTable, TableOpts};
use mtgoupdater::MtgoCard;
use std::{
    fmt::Alignment,
    sync::{Arc, Mutex},
};

use self::column::{SortStates, SortedBy};

use super::TableMessage;

pub mod column;
mod util;
use util::CollectionColumn;

/// [SortToggle] is a button that toggles between ascending and descending sort order
pub struct SortToggle {
    b: button::Button,
}

impl SortToggle {
    /// Create a new [SortToggle] button with the given label and [SortedBy] state
    pub fn new(label: &str, ord: Arc<Mutex<SortedBy>>) -> Self {
        let mut b = button::Button::default()
            .with_size(70, 0)
            .with_label(label)
            .with_align(Align::Center | Align::Inside);
        b.set_selection_color(Color::color_average(b.color(), Color::Foreground, 0.9));
        b.clear_visible_focus();
        b.set_label_size(app::font_size() + 2);
        b.draw(move |b| {
            let ordering = ord.lock().unwrap();

            if !ordering.is_sorted() {
                // Do nothing if not sorted yet
                return;
            }

            let mut image = if ordering.is_descending() {
                get_desc_svg().clone()
            } else {
                get_asc_svg().clone()
            };
            image.scale(15, 15, true, true);
            image.draw(b.x() + (b.w() - 20) + 5, b.y() + 30, b.w(), b.h());
        });
        Self { b }
    }
}

fltk::widget_extends!(SortToggle, button::Button, b);

/// The collection table that displays all cards in the collection
pub struct CollectionTable {
    table: SmartTable,
    cards: Vec<MtgoCard>,
    sort_states: SortStates,
}

impl CollectionTable {
    // Column layout for the collection table
    pub const COL_NAME: CollectionColumn = CollectionColumn::new(0, "NAME", 300);
    pub const COL_QUANTITY: CollectionColumn = CollectionColumn::new(1, "Quantity", 60);
    pub const COL_FOIL: CollectionColumn = CollectionColumn::new(2, "FOIL", 60);
    pub const COL_GOATBOTS: CollectionColumn = CollectionColumn::new(3, "GOATBOTS", 120);
    pub const COL_CARDHOARDER: CollectionColumn = CollectionColumn::new(4, "CARDHOARDER", 120);
    pub const COL_SET: CollectionColumn = CollectionColumn::new(5, "SET", 60);
    pub const COL_RARITY: CollectionColumn = CollectionColumn::new(6, "RARITY", 100);

    /// Create a new [CollectionTable] with the given width, height, and event sender
    pub fn new(w: i32, h: i32, ev_sender: app::Sender<Message>, sort_states: SortStates) -> Self {
        // Create the row of buttons to sort by columns

        // Create the table that displays all cards with their info
        let mut table = SmartTable::default()
            .with_size(w, h)
            .center_of_parent()
            .with_opts(TableOpts {
                rows: 0,
                cols: 7,
                editable: false,
                cell_font_color: Color::White,
                header_frame: FrameType::NoBox,
                header_font_color: Color::White,
                ..Default::default()
            });

        table.set_row_header(false);
        table.set_col_header(false);

        table.set_col_width(Self::COL_NAME.idx, Self::COL_NAME.width);
        table.set_col_width(Self::COL_QUANTITY.idx, Self::COL_QUANTITY.width);
        table.set_col_width(Self::COL_FOIL.idx, Self::COL_FOIL.width);
        table.set_col_width(Self::COL_GOATBOTS.idx, Self::COL_GOATBOTS.width);
        table.set_col_width(Self::COL_CARDHOARDER.idx, Self::COL_CARDHOARDER.width);
        table.set_col_width(Self::COL_SET.idx, Self::COL_SET.width);
        table.set_col_width(Self::COL_RARITY.idx, Self::COL_RARITY.width);

        // Support drag-and-drop a full trade list file
        util::set_drag_and_drop_callback(&mut table, ev_sender);

        Self {
            table,
            cards: vec![],
            sort_states,
        }
    }

    /// Handle the given [TableMessage] event
    pub fn handle_ev(&mut self, ev: TableMessage) {
        match ev {
            TableMessage::SortBy(cat) => {
                util::sort_cards(&mut self.cards, &mut self.sort_states, cat);
                self.draw_cards();
            }
            TableMessage::Search(str) => {
                if self.cards.is_empty() {
                    return;
                }
                let pattern = str.to_lowercase();
                let filtered_cards = self
                    .cards
                    .iter()
                    .filter(|c| c.name.to_lowercase().contains(&pattern));

                let mut filter_count = 0;
                filtered_cards.into_iter().enumerate().for_each(|(idx, c)| {
                    let row_idx = idx as i32;
                    if row_idx > self.table.row_count() - 1 {
                        self.table.append_empty_row("");
                    }
                    util::fill_card_row(&mut self.table, row_idx, c);
                    filter_count = row_idx + 1
                });
                if filter_count < self.table.row_count() {
                    let mut table_row_count = self.table.row_count();
                    let diff = table_row_count - filter_count;
                    for _ in 0..diff {
                        table_row_count -= 1;
                        self.table.remove_row(table_row_count);
                    }
                }
            }
        }
    }

    /// Set the cards to display in the table from the given [MtgoCard]s vector
    pub fn set_cards(&mut self, cards: Vec<MtgoCard>) {
        self.cards = cards;
        self.draw_cards();
    }

    /// Draw/refresh the cards in the table
    fn draw_cards(&mut self) {
        util::draw_cards(&mut self.table, &self.cards);
    }

    /// Filter the cards in the table by the given string
    fn filter_cards_by_str(&mut self, str: &str) {
        // Early return if no cards to filter
        if self.cards.is_empty() {
            return;
        }
        // Filter the cards by the given string in a case-insensitive manner
        let pattern = str.to_lowercase();
        let filtered_cards = self
            .cards
            .iter()
            .filter(|c| c.name.to_lowercase().contains(&pattern));

        // Iterate over the filtered cards and fill the table with them
        let mut filter_count = 0;
        filtered_cards.into_iter().enumerate().for_each(|(idx, c)| {
            let row_idx = idx as i32;
            // Extend the table with rows matching the amount of cards
            if row_idx > self.table.row_count() - 1 {
                self.table.append_empty_row("");
            }
            util::fill_card_row(&mut self.table, row_idx, c);
            filter_count = row_idx + 1
        });

        if filter_count < self.table.row_count() {
            self.remove_excess_rows(filter_count);
        }
    }

    /// Remove the excess rows from the table that are not needed to display the given amount of cards
    ///
    /// Used when filtering the table to remove the rows that are not needed to display the filtered cards
    fn remove_excess_rows(&mut self, display_count: i32) {
        let mut table_row_count = self.table.row_count();
        let diff = table_row_count - display_count;
        for _ in 0..diff {
            table_row_count -= 1;
            self.table.remove_row(table_row_count);
        }
    }
}
