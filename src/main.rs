#![windows_subsystem = "windows"]

mod montagne_theme;

use montagne_theme::{editor_style, text_editor_style};

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::sync::Arc;

use ::iced::widget::text_editor;
use ::iced::{Element, Length};
use iced::widget::{column, container, horizontal_space, row, text};
use iced::{Padding, Task};

fn main() -> iced::Result {
    iced::application("Montagne", Montagne::update, Montagne::view)
        .centered()
        .run_with(Montagne::new)
}

// define state
struct Montagne {
    content: text_editor::Content,
    str_path_last_closed_file: String,
}

// define messages (interactions of the application)
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
}

impl Montagne {
    fn new() -> (Self, Task<Message>) {
        // change later to query from SQLite
        // this whole thing will later evolve into opening a number of files
        // to continue working on them (if they were not closed by user)
        // let str_path_last_closed_file = String::new();
        // let path = Path::new("target/README.md");
        let str_path_last_closed_file = "target/README.md".to_string();

        let path = Path::new(str_path_last_closed_file.as_str());

        /* Use this later to show error message that a file couldn't be opened
        let display = path.display();
         */

        if let Ok(mut file) = File::open(&path) {
            let mut s = String::new();

            if let Ok(_why) = file.read_to_string(&mut s) {
                return (
                    Self {
                        content: text_editor::Content::with_text(&s),
                        str_path_last_closed_file: str_path_last_closed_file,
                    },
                    Task::none(),
                );
            }
        }

        // Couldn't open file for some reason
        (
            Self {
                content: text_editor::Content::default(),
                str_path_last_closed_file: String::default(),
            },
            Task::none(),
        )
    }
        
    fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => {
                self.content.perform(action);
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let menu_bar = row![horizontal_space(), text(&self.str_path_last_closed_file),];

        let text_editor_input = text_editor(&self.content)
            .height(Length::Fill)
            .on_action(Message::Edit)
            .style(text_editor_style);

        let position = {
            let (ln, col) = self.content.cursor_position();

            text(format!("Ln {}, Col {}", ln + 1, col + 1))
        };

        container(column![menu_bar, text_editor_input, position])
            .padding(Padding::from([5, 5]))
            .style(editor_style)
            .into()
    }
}

async fn load_file(path: impl AsRef<Path>) -> Result<Arc<String>, io::ErrorKind> {
    tokio::fs::read_to_string(path)
        .await
        .map(Arc::new)
        .map_err(|error| error.kind())
}
