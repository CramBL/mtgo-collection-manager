use fltk::{
    enums::{Color, Font},
    prelude::{WidgetExt, WindowExt},
    text::{self, TextAttr, TextBuffer},
    window::Window,
};

use crate::menubar::util::TextBufferStylePair;

/// Create the text buffers for the about window
///
/// # Arguments
///
/// * `mtgogui_version` - The version of the MTGO Collection Manager
/// * `project_url` - The URL of the project homepage
///
/// # Returns
///
/// A [TextBufferStylePair] containing the text and style buffers
pub fn fill_about_text_buffers(mtgogui_version: &str, project_url: &str) -> TextBufferStylePair {
    let mut tbuf = TextBuffer::default();
    let mut sbuf = TextBuffer::default();
    let mtgo_cm_txt = format!(
        "{:^width$}\n",
        "MTGO Collection Manager",
        width = 65 - "MTGO Collection Manager".len()
    );
    let mtgo_cm_ver_txt = format!(
        "{:^width$}\n\n",
        format!("v{mtgogui_version}"),
        width = 57 - format!("v{mtgogui_version}").len()
    );
    tbuf.set_text(&mtgo_cm_txt);
    sbuf.set_text(&"A".repeat(mtgo_cm_txt.len()));
    tbuf.append(&mtgo_cm_ver_txt);
    sbuf.append(&"A".repeat(mtgo_cm_ver_txt.len()));
    tbuf.append("Homepage:\n");
    sbuf.append(&"B".repeat("Homepage:\n".len()));
    tbuf.append(project_url);
    sbuf.append(&"D".repeat(project_url.len()));

    TextBufferStylePair::new(tbuf, sbuf)
}

/// Styling for the about window text
pub const TEXT_ABOUT_STYLES: &[text::StyleTableEntryExt] = &[
    text::StyleTableEntryExt {
        color: Color::White,
        font: Font::HelveticaBold,
        size: 20,
        // defaults
        attr: TextAttr::None,
        bgcolor: Color::Background2,
    },
    text::StyleTableEntryExt {
        color: Color::from_hex(0xA8A8A8),
        font: Font::Helvetica,
        size: 18,
        attr: TextAttr::Underline,
        bgcolor: Color::Background2, // default
    },
    text::StyleTableEntryExt {
        color: Color::Yellow,
        font: Font::Courier,
        size: 16,
        // defaults
        attr: TextAttr::None,
        bgcolor: Color::Background2,
    },
    text::StyleTableEntryExt {
        color: Color::DarkBlue,
        font: Font::HelveticaItalic,
        size: 16,
        // defaults
        attr: TextAttr::None,
        bgcolor: Color::Background2,
    },
];
