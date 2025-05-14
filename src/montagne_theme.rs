use iced::Border;
use iced::Color;
use iced::Shadow;
use iced::Theme;
use iced::widget::container;
use iced::widget::text_editor;

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
        selection: Color::WHITE.scale_alpha(0.30),
    }
}
