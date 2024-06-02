use super::MenubarMessage;
use crate::Message;
use fltk::{
    app::Sender,
    enums::{Color, FrameType, Shortcut},
    menu::{self, MenuFlag},
    prelude::{MenuExt, WidgetExt},
};

/// Initialize the menubar, adding all the menu items
pub(super) fn init_menu_bar(menu: &mut menu::SysMenuBar, s: Sender<Message>) {
    menu.set_frame(FrameType::FlatBox);

    menu.add_emit(
        "&File/Open Full Trade list...\t",
        Shortcut::Ctrl | 'o',
        MenuFlag::Normal,
        s.clone(),
        MenubarMessage::Open.into(),
    );

    menu.add_emit(
        "&File/Quit\t",
        Shortcut::Ctrl | 'q',
        MenuFlag::Normal,
        s.clone(),
        Message::Quit,
    );

    menu.add_emit(
        "&Help/About\t",
        Shortcut::None,
        MenuFlag::Normal,
        s.clone(),
        MenubarMessage::About.into(),
    );

    if let Some(mut item) = menu.find_item("&File/Quit\t") {
        item.set_label_color(Color::Red);
    }
}
