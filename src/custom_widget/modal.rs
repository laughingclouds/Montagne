use crate::montagne_theme::modal_style;

use iced::{
    Border, Color, Element,
    border::Radius,
    widget::{button, center, column, container, opaque, row, stack, text},
};

use crate::message::Message;

// This has the where clause to confirm Message is what I want it to be
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
