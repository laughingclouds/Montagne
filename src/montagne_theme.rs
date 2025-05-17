use iced::widget::{container, scrollable, text, text_editor};
use iced::{Border, Color, Element, Font, Shadow, Theme};

// styling
pub fn editor_style(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::WHITE),
        background: Some(iced::Background::Color(Color::from_rgb(0.10, 0.10, 0.10))),
        ..container::Style::default()
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

pub fn preview_scrollable_style(_theme: &Theme, status: scrollable::Status) -> scrollable::Style {
    let transparent_border = Border {
        color: Color::TRANSPARENT,
        ..Border::default()
    };

    let transparent_scroller = scrollable::Scroller {
        color: Color::TRANSPARENT,
        border: transparent_border,
    };

    let transparent_rail = scrollable::Rail {
        background: Some(iced::Background::Color(Color::TRANSPARENT)),
        border: transparent_border,
        scroller: transparent_scroller,
    };

    let transparent_style = scrollable::Style {
        container: container::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            border: transparent_border,
            shadow: Shadow {
                color: Color::TRANSPARENT,
                ..Shadow::default()
            },
            ..container::Style::default()
        },
        vertical_rail: transparent_rail,
        horizontal_rail: transparent_rail,
        gap: Some(iced::Background::Color(Color::TRANSPARENT)),
    };

    let default_scroller = scrollable::Scroller {
        color: Color::default(),
        border: Border::default(),
    };

    let default_rail = scrollable::Rail {
        background: Some(iced::Background::Color(Color::default())),
        border: Border::default(),
        scroller: default_scroller,
    };

    match status {
        scrollable::Status::Hovered {
            is_horizontal_scrollbar_hovered: _,
            is_vertical_scrollbar_hovered: _,
        } => scrollable::Style {
            container: container::Style::default(),
            vertical_rail: default_rail,
            horizontal_rail: default_rail,
            gap: Some(iced::Background::Color(Color::default())),
        },
        scrollable::Status::Active => transparent_style,
        scrollable::Status::Dragged {
            is_horizontal_scrollbar_dragged: _,
            is_vertical_scrollbar_dragged: _,
        } => transparent_style,
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
