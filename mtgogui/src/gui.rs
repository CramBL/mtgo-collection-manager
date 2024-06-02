use std::ffi::OsStr;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::appdata::state::GuiState;
use crate::assets::{self, get_asc_svg, get_icon_search, get_logo};
use crate::collection::processor::TradelistProcessor;
use crate::collection::stats::items::BrowserItems;
use crate::collection::stats::view::StatsView;
use crate::collection::view::table::CollectionTable;
use crate::collection::TableMessage;
use crate::ctrlc_handler::init_ctrlc_handler;
use crate::menubar::McmMenuBar;
use crate::util::first_file_match_from_dir;
use crate::{
    appdata, collection, Message, DEFAULT_APP_HEIGHT, DEFAULT_APP_WIDTH, MENU_BAR_HEIGHT,
    MIN_APP_HEIGHT, MIN_APP_WIDTH,
};
use fltk::enums::{Align, CallbackTrigger, Event, Font, FrameType, Shortcut};
use fltk::frame::Frame;
use fltk::image::{Image, PngImage, TiledImage};
use fltk::misc::Progress;
use fltk::prelude::WidgetExt;
use fltk::text::{TextAttr, TextBuffer, TextDisplay, WrapMode};
use fltk::window::DoubleWindow;
use fltk::{app, button, enums::Color, prelude::*, window::Window};
use fltk::{prelude::*, *};
use fltk_flex::{Flex, FlexType};
use fltk_grid::Grid;
use fltk_table::{SmartTable, TableOpts};
use fltk_theme::{widget_themes, ThemeType, WidgetTheme};
use mtgogetter::fetch_log::CardInfoMetaData;

use self::setup::setup_main_window;

mod setup;

/// [MtgoGui] is the main GUI struct that holds all the widgets and state for the application
pub struct MtgoGui {
    app: app::App,
    state: GuiState,
    rcv: app::Receiver<Message>,
    main_win: window::Window,
    menu: McmMenuBar,
    collection: CollectionTable,
    collection_stats: StatsView,
    metadata: StatsView,
    tradelist_processor: TradelistProcessor,
}

impl Default for MtgoGui {
    fn default() -> Self {
        Self::new()
    }
}

impl MtgoGui {
    /// Create a new [MtgoGui] instance
    pub fn new() -> Self {
        let app = app::App::default();
        let (ev_send, ev_rcv) = app::channel();
        init_ctrlc_handler(ev_send.clone());
        let theme = WidgetTheme::new(ThemeType::Dark);
        theme.apply();
        misc::Tooltip::set_text_color(Color::Black);

        let mut main_win: DoubleWindow = setup::setup_main_window();

        let menu = McmMenuBar::new(DEFAULT_APP_WIDTH, MENU_BAR_HEIGHT, ev_send.clone());

        let mut flx_left_col = setup::setup_left_column_flx_box();

        let search_box = setup::set_search_box(ev_send.clone());
        flx_left_col.fixed(&search_box, 30);

        let collection_stats = StatsView::default();
        let metadata = StatsView::default();

        flx_left_col.end();

        let collection = collection::view::set_collection_main_box(ev_send.clone());

        main_win.end();
        main_win.show();

        let tradelist_processor = TradelistProcessor::new(ev_send.clone());

        main_win.set_callback(move |_| {
            if app::event() == Event::Close {
                ev_send.send(Message::Quit);
            }
        });
        Self {
            app,
            state: GuiState::default(), // Placeholder, is overwritten at startup
            rcv: ev_rcv,
            main_win,
            menu,
            collection,
            collection_stats,
            metadata,
            tradelist_processor,
        }
    }

    /// Perform any startup tasks.
    ///
    /// Runs after all the GUI elements are created. And just before the main event loop starts.
    fn run_startup(&mut self) -> Result<(), String> {
        let appdata_dir = match appdata::util::appdata_path() {
            Ok(appdata_dir) => appdata_dir,
            Err(e) => return Err(format!("Failed to get appdata path: {e}")),
        };

        self.state = match GuiState::load(appdata_dir.clone()) {
            Ok(state) => state,
            Err(e) => {
                log::warn!("Failed to load GUI state: {e}");
                GuiState::default()
            }
        };

        self.set_metadata_browser_view(appdata_dir.clone())?;

        log::info!("Processing current tradelist");
        match appdata::util::current_tradelist_path() {
            Ok(Some(current_trade_list)) => {
                self.tradelist_processor.process(current_trade_list.into())
            }
            Err(e) => {
                // TODO - Error pop-up dialog if fails.
                return Err(format!("Failed to get current tradelist path: {e}"));
            }
            Ok(None) => {
                log::info!("No current trade list found");
            }
        }
        Ok(())
    }

    /// Run the application.
    ///
    /// Runs the startup tasks and the main event loop.
    ///
    /// This function will block until the application is closed.
    pub fn run(&mut self) {
        log::info!("Running startup");
        if let Err(e) = self.run_startup() {
            log::error!("Failed to run startup: {e}");
            self.app.quit();
        }
        log::info!("Running main event loop");
        self.gui_main_event_loop();
    }

    /// The main event loop for the application
    fn gui_main_event_loop(&mut self) {
        while self.app.wait() {
            if let Some(msg) = self.rcv.recv() {
                match msg {
                    Message::Quit => {
                        log::info!("Quit");
                        if let Err(e) = self.state.save(
                            appdata::util::appdata_path().expect("Failed to get appdata path"),
                        ) {
                            log::error!("Failed to save GUI state: {e}");
                        }
                        self.app.quit();
                    }
                    Message::MenuBar(mb_msg) => self.menu.handle_ev(mb_msg),

                    Message::Table(t_m) => {
                        self.collection.handle_ev(t_m);
                        self.app.redraw();
                    }
                    Message::GotFullTradeList(full_trade_list_path) => {
                        // TODO: Error pop-up dialog if fails.
                        // Should implement a generic error dialog that can be used for all unexpected errors that cannot be handled programmatically.
                        appdata::util::copy_tradelist_to_appdata(full_trade_list_path.as_os_str())
                            .unwrap();
                        self.state.new_tradelist();

                        self.tradelist_processor.process(full_trade_list_path);
                    }
                    Message::SetCards(cards) => self.collection.set_cards(cards),
                    Message::SetCollectionStats(stats) => {
                        match stats.try_into() {
                            Ok(browser_items) => {
                                self.collection_stats.set_items(browser_items);
                            }
                            Err(e) => {
                                log::error!("Failed to convert stats to browser items: {e}");
                            }
                        }
                        match appdata::util::appdata_path() {
                            Ok(appdata_dir) => {
                                let _ = self.set_metadata_browser_view(appdata_dir).map_err(|e| {
                                    log::error!("Failed to set metadata browser view: {e}");
                                });
                            }
                            Err(e) => {
                                log::error!("Failed to get appdata path: {e}");
                            }
                        }
                    }
                }
            }
        }
    }
}

// Utility functions

impl MtgoGui {
    fn set_metadata_browser_view(&mut self, appdata_dir: PathBuf) -> Result<(), String> {
        let tradelist_added_date_str: Option<String> =
            if let Some(tradelist_added_date) = self.state.get_tradelist_added_date() {
                Some(format!("{}", tradelist_added_date.format("%-d %B, %C%y")))
            } else {
                log::warn!("No tradelist added date found");
                None
            };

        let mut metadata_browser_items = BrowserItems::new();
        metadata_browser_items.add_item(
            "dek-File added",
            tradelist_added_date_str
                .as_deref()
                .unwrap_or("No tradelist added"),
        );

        log::info!("Loading metadata");

        match CardInfoMetaData::from_toml_file(
            appdata_dir.join(CardInfoMetaData::FILENAME).as_path(),
        ) {
            Ok(metadata) => {
                let mut items: BrowserItems = match metadata.try_into() {
                    Ok(browser_items) => browser_items,
                    Err(e) => {
                        return Err(format!("Failed to convert metadata to browser items: {e}"))
                    }
                };
                metadata_browser_items.append(&mut items);
            }

            Err(e) => {
                // On startup this is a warning, as this is generated from MTGO Getter download data.
                //  if it fails after MTGO Getter has been running, it is an error.
                log::warn!("Failed to load metadata: {e}");
            }
        };

        self.metadata.set_items(metadata_browser_items);
        Ok(())
    }
}
