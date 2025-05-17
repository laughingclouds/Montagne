use iced::widget::{container, text, text_editor};
use iced::{Border, Color, Element, Font, Shadow, Theme};

// styling
pub fn editor_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(Color::from_rgb(0.10, 0.10, 0.10))),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

pub fn text_editor_style(_theme: &Theme, _status: text_editor::Status) -> text_editor::Style {
    text_editor::Style {
        background: iced::Background::Color(Color::from_rgb(0.1216, 0.1216, 0.1216)),
        border: Border::default(),
        icon: Color::default(),
        placeholder: Color::WHITE,
        value: Color::WHITE,
        selection: Color::from_rgb(0.6784, 0.8392, 1.0).scale_alpha(0.15),
    }
}

pub fn exit_modal_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: (Some(
            Color {
                a: 0.8,
                ..Color::BLACK
            }
            .into(),
        )),
        ..container::Style::default()
    }
}

// directly copied from https://github.com/iced-rs/iced/blob/9bfbd7cda79aceef2d115b8bb35e8f3257dcabf2/examples/editor/src/main.rs#L306

pub fn new_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e800}')
}

pub fn save_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0e801}')
}

pub fn open_icon<'a, Message>() -> Element<'a, Message> {
    icon('\u{0f115}')
}

pub fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON_FONT: Font = Font::with_name("editor-icons");

    text(codepoint).font(ICON_FONT).into()
}
