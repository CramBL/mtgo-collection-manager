use crate::{assets, util::center, Message, DEFAULT_APP_WIDTH, MENU_BAR_HEIGHT, MIN_APP_WIDTH};
use fltk::{
    app::{self, Sender},
    dialog::{self, FileDialog, FileDialogOptions, FileDialogType},
    enums::{self, Align, Color, Font, FrameType, Shortcut},
    menu::{self, MenuFlag},
    misc::Progress,
    prelude::*,
    text::{TextAttr, TextBuffer, TextDisplay, WrapMode},
    window::Window,
};
use fltk_flex::Flex;
use std::{io, path::PathBuf};
use util::ProgressUpdate;

mod about;
mod setup;
pub mod util;

/// Messages that can be received by the menubar
#[derive(Debug, Clone)]
pub enum MenubarMessage {
    Open,
    Quit,
    About,
    Example,
    ProgressBar(ProgressUpdate),
}

/// The menubar for the application
pub struct McmMenuBar {
    menu: menu::SysMenuBar,
    ev_emitter: app::Sender<Message>,
    progress_bar: Progress,
}

impl McmMenuBar {
    pub const PROGRESS_BAR_WIDTH: i32 = 300;

    /// Creates a new menubar
    ///
    /// # Arguments
    ///
    /// * `w` - Width of the menubar
    /// * `h` - Height of the menubar
    /// * `s` - Sender to send messages to the main thread
    pub fn new(w: i32, h: i32, s: app::Sender<Message>) -> Self {
        let mut mb = menu::SysMenuBar::default().with_size(w, h);
        setup::init_menu_bar(&mut mb, s.clone());

        let mut progress = Progress::new(
            DEFAULT_APP_WIDTH - Self::PROGRESS_BAR_WIDTH,
            0,
            Self::PROGRESS_BAR_WIDTH,
            MENU_BAR_HEIGHT,
            "",
        );
        progress.set_selection_color(Color::Green);
        progress.set_frame(FrameType::FlatBox);
        progress.set_color(Color::Background2);
        progress.set_maximum(100.);
        progress.set_value(0.);
        progress.set_align(enums::Align::Left | enums::Align::Inside);
        progress.set_label_font(Font::Screen);
        progress.hide();

        Self {
            menu: mb,
            ev_emitter: s,
            progress_bar: progress,
        }
    }

    /// Handles events sent to the menubar
    ///
    /// # Arguments
    ///
    /// * `ev` - The event to handle
    pub fn handle_ev(&mut self, ev: MenubarMessage) {
        match ev {
            MenubarMessage::Open => self.open_full_tradelist(),
            MenubarMessage::Quit => app::quit(),
            MenubarMessage::About => about::show_about(),
            MenubarMessage::Example => todo!("example"),
            MenubarMessage::ProgressBar(update) => {
                if update.show {
                    self.progress_bar.set_value(update.progress);
                    self.progress_bar
                        .set_label(&format!("     {}", update.label));
                    self.progress_bar
                        .set_selection_color(update.selection_color);
                    self.progress_bar.resize(
                        self.menu.width() - update.rel_size().rel_val_x(Self::PROGRESS_BAR_WIDTH),
                        0,
                        update.rel_size().rel_val_w(Self::PROGRESS_BAR_WIDTH),
                        MENU_BAR_HEIGHT,
                    );
                    self.progress_bar.redraw_label();
                    self.progress_bar.redraw();
                    self.progress_bar.show();
                    app::redraw();
                } else {
                    self.progress_bar.hide();
                }
            }
        }
    }

    fn open_full_tradelist(&mut self) {
        let mut dlg = FileDialog::new(FileDialogType::BrowseFile);
        dlg.set_option(FileDialogOptions::NoOptions);
        dlg.set_filter("MTGO Full Trade List\t*.{txt,dek}");
        dlg.show();
        let filename = dlg.filename();
        if !filename.to_string_lossy().to_string().is_empty() {
            if filename.is_file() {
                log::info!("Full trade list: {filename:?}");
                self.ev_emitter
                    .send(Message::GotFullTradeList(filename.into()));
            } else {
                dialog::alert(center().0 - 200, center().1 - 100, "File does not exist!")
            }
        }
    }
}
