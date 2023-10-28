#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(dead_code)]

use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use assets::{get_asc_svg, get_icon_search, get_logo};
use flexi_logger::{Cleanup, Criterion, Duplicate, Naming};
use flexi_logger::{FileSpec, Logger, WriteMode};
use fltk::enums::{CallbackTrigger, Event, Font, FrameType, Shortcut};
use fltk::image::{Image, PngImage, TiledImage};
use fltk::prelude::WidgetExt;
use fltk::text::TextAttr;
use fltk::window::DoubleWindow;
use fltk::{app, button, enums::Color, prelude::*, window::Window};
use fltk::{prelude::*, *};
use fltk_flex::{Flex, FlexType};
use fltk_grid::Grid;
use fltk_table::{SmartTable, TableOpts};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};

mod appdata;
mod assets;
mod collection;
mod gui;
mod menubar;
mod util;

use collection::view::table;
use collection::view::table::column;
use collection::TableMessage;
use gui::MtgoGui;
use menubar::McmMenuBar;
use mtgoupdater::mtgo_card::MtgoCard;
use mtgoupdater::mtgo_preprocessor_api::run_mtgo_preprocessor_version;
use mtgoupdater::mtgogetter_api::mtgogetter_version;

use crate::util::center;

pub const MIN_APP_WIDTH: i32 = 400;
pub const MIN_APP_HEIGHT: i32 = 400;
pub const DEFAULT_APP_WIDTH: i32 = 1400;
pub const DEFAULT_APP_HEIGHT: i32 = 800;
pub const WIDGET_PADDING: i32 = 0;

pub const MENU_BAR_HEIGHT: i32 = 25;

/// Messages for the main event loop
#[derive(Debug, Clone)]
pub enum Message {
    Quit,
    Example,
    MenuBar(menubar::MenubarMessage),
    Table(collection::TableMessage),
    GotFullTradeList(Box<Path>),
    SetCards(Vec<MtgoCard>),
}

/// Conversion from [menubar::MenubarMessage] to [Message]
impl From<menubar::MenubarMessage> for Message {
    fn from(mb_msg: menubar::MenubarMessage) -> Self {
        Message::MenuBar(mb_msg)
    }
}

/// Conversion from [collection::TableMessage] to [Message]
impl From<collection::TableMessage> for Message {
    fn from(ct_msg: collection::TableMessage) -> Self {
        Message::Table(ct_msg)
    }
}

fn main() {
    // In debug mode use the paths to the binaries when they're built in each subproject
    if cfg!(debug_assertions) {
        mtgoupdater::internal_only::dev_try_init_mtgogetter_bin();
        mtgoupdater::internal_only::dev_try_init_mtgoparser_bin();
        // Show box edges
        Flex::debug(true);
    }
    // Setup logger (has to be done with a let binding to make the logger live long enough)
    let _logger = util::setup_logger();
    log::info!("Setup GUI");
    let mut gui = MtgoGui::default();
    log::info!("Starting GUI");
    gui.run();
}
