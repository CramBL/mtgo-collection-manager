use std::{default, io};

use fltk::{
    enums::{Color, Font},
    text::{self, TextAttr, TextBuffer},
};

use crate::{util::RelativeSize, DEFAULT_APP_WIDTH, MENU_BAR_HEIGHT};

use super::McmMenuBar;

/// Progress bar update message
#[derive(Debug, Clone)]
pub struct ProgressUpdate {
    pub show: bool,
    pub progress: f64,
    pub label: Box<str>,
    pub selection_color: Color,
    pub rel_size: RelativeSize,
}

impl default::Default for ProgressUpdate {
    fn default() -> Self {
        Self::new(false, 0., "".into(), Color::Green, RelativeSize::default())
    }
}

impl ProgressUpdate {
    /// Create a new [ProgressUpdate] instance. Allowing updating the appearance of the progress bar.
    /// Updating relative values means scaling the progress bar relative to its current position and/or size.
    ///
    /// # Arguments
    ///
    /// * `show` - Whether or not to show the progress bar
    /// * `progress` - The progress of the progress bar
    /// * `label` - The label to display on the progress bar
    /// * `selection_color` - The color that fills the progress bar
    /// * `relative_size` - The relative position and size of the progress bar
    pub fn new(
        show: bool,
        progress: f64,
        label: Box<str>,
        selection_color: Color,
        relative_size: RelativeSize,
    ) -> Self {
        Self {
            show,
            progress,
            label,
            selection_color,
            rel_size: relative_size,
        }
    }

    /// Get the relative position and size to perform a [ProgressUpdate] with
    pub fn rel_size(&self) -> &RelativeSize {
        &self.rel_size
    }
}

/// A text buffer and its associated style buffer
#[derive(Debug)]
pub struct TextBufferStylePair {
    text: Option<TextBuffer>,
    style: Option<TextBuffer>,
}

impl TextBufferStylePair {
    /// Create a new [TextBufferStylePair] with the given text and style buffers
    pub fn new(text: TextBuffer, style: TextBuffer) -> Self {
        Self {
            text: Some(text),
            style: Some(style),
        }
    }

    /// Take the text buffer out of the [TextBufferStylePair]
    pub fn text(&mut self) -> TextBuffer {
        self.text.take().expect("Text buffer already taken")
    }

    /// Take the style buffer out of the [TextBufferStylePair]
    pub fn style(&mut self) -> TextBuffer {
        self.style.take().expect("Style buffer already taken")
    }

    /// Get the number of lines in the text buffer
    ///
    /// # Panics
    ///
    /// Panics if any of the buffers have already been taken
    pub fn line_count(&self) -> i32 {
        self.text
            .as_ref()
            .expect("Text buffer already taken")
            .count_lines(
                0,
                self.text
                    .as_ref()
                    .expect("Style buffer already taken")
                    .length(),
            )
    }
}
