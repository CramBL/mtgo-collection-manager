use std::{
    ffi::OsStr,
    io::Error,
    path::{Path, PathBuf},
    thread,
};

use fltk::{app::Sender, enums::Color};

use crate::{
    appdata::{update::AppData, APP_DATA_DIR},
    collection::stats::container::CollectionStats,
    menubar::util::ProgressUpdate,
    menubar::{McmMenuBar, MenubarMessage},
    util::{first_file_match_from_dir, RelativeSize},
    Message, DEFAULT_APP_WIDTH, MENU_BAR_HEIGHT,
};

/// [TradelistProcessor] is responsible for processing the tradelist, updating the card data, and assigning the cards to the collection table.
#[derive(Debug)]
pub struct TradelistProcessor {
    event_sender: Sender<Message>,
}

impl TradelistProcessor {
    /// Create a new [TradelistProcessor] instance
    pub fn new(ev_send: Sender<Message>) -> Self {
        Self {
            event_sender: ev_send,
        }
    }

    /// Process the tradelist at the given path
    ///
    /// # Arguments
    ///
    /// * `full_trade_list_path` - [Path] to the full trade list
    pub fn process(&mut self, full_trade_list_path: Box<Path>) {
        // TODO: Some basic verification that we actually got a trade list and not some random non-sense.
        let trade_list_processor_thread =
            std::thread::Builder::new().name("Trade List Processor".to_string());
        log::info!("Spawning Trade List Processor thread");
        let _handle = trade_list_processor_thread
            .spawn({
                let sender = self.event_sender.clone();
                move || {
                    sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                        ProgressUpdate {
                            show: true,
                            progress: 5.,
                            label: "Processing trade list".into(),
                            ..Default::default()
                        },
                    )));

                    // Invoke MTGO getter
                    sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                        ProgressUpdate {
                            show: true,
                            progress: 10.,
                            label: "Updating card data...".into(),
                            ..Default::default()
                        },
                    )));

                    // Give the full trade list to the parser
                    // Find all the most recent files in the appdata directory, download and update them if necessary
                    let appdata_paths = match AppData::update() {
                        Ok(paths) => paths,
                        Err(err) => {
                            log::info!("{err}");
                            return;
                        }
                    };

                    sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                        ProgressUpdate {
                            show: true,
                            progress: 70.,
                            label: "Update done".into(),
                            ..Default::default()
                        },
                    )));

                    log::info!("Using MTGO Parser");
                    log::info!("Scryfall path: {p:?}", p = appdata_paths.scryfall_path());
                    log::info!(
                        "Card definitions path: {p:?}",
                        p = appdata_paths.card_definitions_path()
                    );
                    log::info!(
                        "Price history path: {p:?}",
                        p = appdata_paths.price_history_path()
                    );
                    log::info!("Save to dir: {p:?}", p = appdata_paths.appdata_dir_path());

                    sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                        ProgressUpdate {
                            show: true,
                            progress: 75.,
                            label: "Processing card data...".into(),
                            ..Default::default()
                        },
                    )));

                    match mtgoupdater::parse_full(
                        full_trade_list_path.as_ref(),
                        appdata_paths.card_data.scryfall_path(),
                        appdata_paths.card_data.card_definitions_path(),
                        appdata_paths.card_data.price_history_path(),
                        Some(appdata_paths.appdata_dir.as_path()),
                    ) {
                        Ok(cards) => {
                            log::info!("MTGO Parser-rs output: {} cards", cards.len());
                            // Fill the progress bar as appropriate
                            // Give all the data to the collection table
                            sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                                ProgressUpdate {
                                    show: true,
                                    progress: 95.,
                                    label: "Processing complete...".into(),
                                    ..Default::default()
                                },
                            )));

                            fadeout_progress_bar(sender.clone());
                            sender.send(Message::SetCollectionStats(CollectionStats::from_cards(
                                &cards,
                            )));
                            sender.send(Message::SetCards(cards));
                        }
                        Err(e) => {
                            log::info!("MTGO Parser error: {e}");
                        }
                    }
                }
            })
            .expect("Failed spawning Trade List Processor thread");
    }
}

/// Spawn a thread to set the progress bar to 100% and then fade it out.
///
/// # Arguments
///
/// * `ev_sender` - [Sender] to send the [Message] to
fn fadeout_progress_bar(ev_sender: Sender<Message>) {
    thread::spawn({
        move || {
            // Fade and slide right effect

            // Working with integers so we need to scale the values to get a fraction
            const PERCENT: i32 = 100;
            const CEILING: i32 = 255;
            for i in 1..=255 {
                thread::sleep(std::time::Duration::from_millis(10));
                ev_sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                    ProgressUpdate {
                        show: true,
                        progress: 100.,
                        label: match i {
                            0..=31 => "Collection updated".into(),
                            32..=63 => "Collection updated.".into(),
                            64..=127 => "Collection updated..".into(),
                            128..=142 => "Collection updated...".into(),
                            // Add this point the text switches from black to white, so we just remove it
                            //  otherwise it has a slight *flash* effect which is distracting
                            143..=158 => "                  ...".into(),
                            159..=190 => "                   ..".into(),
                            191..=222 => "                    .".into(),
                            223..=255 => "".into(),
                        },
                        selection_color: Color::from_rgba_tuple((0, 255, 0, 255 - i)),
                        rel_size: RelativeSize {
                            perc_rel_pos_x: PERCENT - ((i as i32 * PERCENT) / CEILING),
                            perc_rel_size_w: PERCENT - ((i as i32 * PERCENT) / CEILING),
                            ..Default::default()
                        },
                    },
                )));
            }
            // Finally hide it
            ev_sender.send(Message::MenuBar(MenubarMessage::ProgressBar(
                ProgressUpdate {
                    show: false,
                    ..Default::default()
                },
            )));
        }
    });
}
