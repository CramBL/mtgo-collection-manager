pub mod processor;
pub mod stats;
pub mod view;

use crate::Message;
use fltk::{
    app::{self, App},
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
use view::table;

/// [TableMessage] is a message/event that can be sent to the collection table
#[derive(Debug, Clone)]
pub enum TableMessage {
    SortBy(table::column::Column),
    Search(Box<str>),
}
