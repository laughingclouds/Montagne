#![windows_subsystem = "windows"]

use ::iced::Element;
use ::iced::widget::text_editor;

// define state
#[derive(Default)]
struct MdEditor {
    content: text_editor::Content,
}

// define messages (interactions of the application)
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl MdEditor {
    fn view(&self) -> Element<'_, Message> {
        text_editor(&self.content).on_action(Message::Edit).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }
}

fn main() -> iced::Result {
    iced::run("mdEdit", MdEditor::update, MdEditor::view)
}
