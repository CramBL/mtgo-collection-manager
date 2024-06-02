use fltk::{
    dialog,
    enums::{Align, Color, Font},
    prelude::{DisplayExt, GroupExt, WidgetExt, WindowExt},
    text::{TextAttr, TextBuffer, TextDisplay, WrapMode},
    window::Window,
};
use fltk_flex::Flex;

use crate::{assets, util::center};

use self::text::TEXT_ABOUT_STYLES;

use super::util::TextBufferStylePair;

pub mod text;

/// Displays the version of the MTGO Collection Manager, MTGO Getter, and MTGO Updater.
pub fn show_about() {
    format_about_window(
        env!("CARGO_PKG_VERSION"),
        "https://github.com/CramBL/mtgo-collection-manager/",
    );
}

/// Create the about window
///
/// # Arguments
///
/// * `mtgogui_version` - The version of the MTGO Collection Manager
/// * `project_url` - The URL of the project homepage
pub fn format_about_window(mtgogui_version: &str, project_url: &str) {
    let txt_buffers = text::fill_about_text_buffers(mtgogui_version, project_url);

    let line_buf_len = txt_buffers.line_count(); //50
    const LINE_BUF_LEN_MULTIPLIER: i32 = 10;
    const BASE_HEIGHT: i32 = 100;
    let about_window_height = BASE_HEIGHT + (line_buf_len * LINE_BUF_LEN_MULTIPLIER);
    log::trace!("About Window height calculation: {BASE_HEIGHT} + ({line_buf_len} * {LINE_BUF_LEN_MULTIPLIER}) = {about_window_height}");
    let mut win = create_about_window(450, about_window_height, mtgogui_version);

    let flex_about = Flex::default()
        .with_pos(0, 0)
        .with_align(Align::Center)
        .size_of_parent()
        .column();

    let _txt_disp = create_about_txt_display(txt_buffers);

    flex_about.end();
    win.end();
    win.show();
}

/// Create the about window
///
/// # Arguments
///
/// * `w` - The width of the window
/// * `h` - The height of the window
/// * `mtgogui_version` - The version of the MTGO Collection Manager
///
/// # Returns
///
/// The about [Window]
pub fn create_about_window(w: i32, h: i32, mtgogui_version: &str) -> Window {
    let mut win = Window::default()
        .with_size(w, h)
        .with_pos(center().0 - 300, center().1 - 100)
        .with_label(&format!(
            "About MTGO Collection Manager v{}",
            mtgogui_version
        ));
    win.set_icon(Some(assets::get_logo()));
    win
}

/// Create the text buffers for the about window
///
/// # Arguments
///
/// * `txt_buffers` - The [TextBufferStylePair] containing the text and style buffers
///
/// # Returns
///
/// The [TextDisplay] containing the text from the buffers
///
/// # Panics
///
/// Panics if the text buffers are not set
pub fn create_about_txt_display(mut txt_buffers: TextBufferStylePair) -> TextDisplay {
    let mut txt_disp = TextDisplay::default();
    txt_disp.align();
    txt_disp.set_buffer(txt_buffers.text());
    txt_disp.set_highlight_data_ext(txt_buffers.style(), TEXT_ABOUT_STYLES);
    txt_disp.set_align(Align::Center);
    txt_disp.wrap_mode(WrapMode::AtBounds, 0);
    txt_disp.set_text_color(Color::White);
    txt_disp
}
