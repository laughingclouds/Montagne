use std::path::PathBuf;

use iced::{
    Border, Color, Element,
    border::Radius,
    widget::{button, center, column, container, opaque, row, stack, text, tooltip},
};

use crate::{message::Message, montagne_theme::modal_style};

// This has the where clause to confirm Message is what I want it to be
/// Generic implementation for stacking `content` on top of `base`.
/// Contains styling as well.
fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            center(opaque(container(content).style(|_them| container::Style {
                text_color: Some(Color::WHITE),
                border: Border {
                    color: Color::WHITE,
                    width: 1.0,
                    radius: Radius::new(8.0)
                },
                ..container::Style::default()
            })))
            .style(modal_style)
        ),
    ]
    .into()
}

// This needs to use message::Message for specific behavior
/// Stack file exit dialogue on top of base content.
pub fn exit_modal<'a>(base: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
    modal(
        base,
        column![
            text("You have unsaved work. Save changes?"),
            row![
                button("Save").on_press(Message::SaveFile),
                button("Close without saving").on_press(Message::CloseApp),
                button("Go back").on_press(Message::CloseExitModal),
            ]
            .spacing(10)
        ]
        .spacing(10)
        .padding(30),
    )
}

/// In case file is dirty (is_dirty == true) while base content of file has changed, show user
/// this modal to ask them what to do.
pub fn file_changed_modal<'a>(
    base: impl Into<Element<'a, Message>>,
    file_path: impl Into<PathBuf>,
) -> Element<'a, Message> {
    let path = file_path.into();

    modal(
        base,
        column![
            text(format!(
                "Changes were to {}. Reload file or keep your changes?",
                path.display()
            )),
            tooltip(
                button("Keep my changes").on_press(Message::SaveFile),
                "This will also write your changes to the file.",
                tooltip::Position::Top
            )
            .style(container::rounded_box),
            button("Reload file").on_press(Message::LoadFile),
        ],
    )
}
