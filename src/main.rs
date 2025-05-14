#![windows_subsystem = "windows"]

use std::fs::File;
use std::io::Read;
use std::path::Path;

use ::iced::{Element, Length};
use ::iced::widget::text_editor;

// define state
struct MdEditor {
    content: text_editor::Content,
    str_path_last_closed_file: String,
}

// TODO: Extend later to reopen file
impl Default for MdEditor {
    fn default() -> Self {
        // change later to query from SQLite
        // this whole thing will later evolve into opening a number of files
        // to continue working on them (if they were not closed by user)
        let str_path_last_closed_file = String::new();
        // let path = Path::new("target/README.md");
        let path = Path::new(str_path_last_closed_file.as_str());

        /* Use this later to show error message that a file couldn't be opened
        let display = path.display();
         */

        if let Ok(mut file) = File::open(&path) {
            let mut s = String::new();

            if let Ok(_why) = file.read_to_string(&mut s) {
                return Self {
                    content: text_editor::Content::with_text(&s),
                    str_path_last_closed_file: str_path_last_closed_file,
                };
            }
        }
        // Couldn't open file for some reason
        Self {
            content: text_editor::Content::default(),
            str_path_last_closed_file: String::default(),
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
        text_editor(&self.content).height(Length::Fill).on_action(Message::Edit).into()
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
