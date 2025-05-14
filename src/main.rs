#![windows_subsystem = "windows"]

use std::fs::File;
use std::io::Read;
use std::path::Path;

use ::iced::Element;
use ::iced::widget::text_editor;

// define state
struct MdEditor {
    content: text_editor::Content,
}

// TODO: Extend later to reopen file
impl Default for MdEditor {
    fn default() -> Self {
        let path = Path::new("target/README.md");

        /* Use this later to show error message that a file couldn't be opened
        let display = path.display();
         */

        if let Ok(mut file) = File::open(&path) {
            let mut s = String::new();

            if let Ok(_why) = file.read_to_string(&mut s) {
                return Self {
                    content: text_editor::Content::with_text(&s),
                };
            }
        }
        // Couldn't open file for some reason
        Self {
            content: text_editor::Content::default(),
        }
    }
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
    iced::run("Montagne", MdEditor::update, MdEditor::view)
}
