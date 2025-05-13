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

impl Default for MdEditor {
    fn default() -> Self {
        let path = Path::new("target/README.md");
        let display = path.display();

        let mut file = match File::open(&path) {
            Err(why) => panic!("Couldn't open {}: {}", display, why),
            Ok(file) => file,
        };

        let mut s = String::new();

        if let Err(why) = file.read_to_string(&mut s) {
            panic!("Couldn't read {}: {}", display, why);
        }

        Self {
            content: text_editor::Content::with_text(&s),
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
