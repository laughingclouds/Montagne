pub mod modal;

use iced::{
    Element,
    widget::{button, container, tooltip},
};

// A wrapper for any on_press events on buttons.
pub fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(30));

    if let Some(on_press) = on_press {
        tooltip(action.on_press(on_press), label, tooltip::Position::Bottom)
            .style(container::rounded_box)
            .into()
    } else {
        action.style(button::secondary).into()
    }
}
