// #![windows_subsystem = "windows"]

mod montagne_theme;

use montagne_theme::{editor_style, text_editor_style};

use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use ::iced::widget::text_editor;
use ::iced::{Element, Length};
use iced::widget::{self, column, container, horizontal_space, markdown, row, scrollable, text};
use iced::{Padding, Task, Theme};

fn main() -> iced::Result {
    iced::application("Montagne", Montagne::update, Montagne::view)
        .centered()
        .theme(Montagne::theme)
        .run_with(Montagne::new)
}

// define state
struct Montagne {
    content: text_editor::Content,
    items: Vec<markdown::Item>,
    file: Option<PathBuf>,
    theme: Theme,
}

// define messages (interactions of the application)
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    LinkClicked(markdown::Url),
}

impl Montagne {
    fn new() -> (Self, Task<Message>) {
        let theme = Theme::Dark;
        (
            Self {
                content: text_editor::Content::new(),
                items: markdown::parse("").collect(),
                file: None,
                theme: theme,
            },
            Task::batch([
                Task::perform(
                    load_file(format!("{}\\target\\README.md", env!("CARGO_MANIFEST_DIR"))),
                    Message::FileOpened,
                ),
                widget::focus_next(),
            ]),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                let is_edit = action.is_edit();

                self.content.perform(action);

                if is_edit {
                    self.items = markdown::parse(&self.content.text()).collect();
                }

                Task::none()
            }
            Message::FileOpened(result) => {
                if let Ok((path, contents)) = result {
                    self.file = Some(path);
                    self.content = text_editor::Content::with_text(&contents);
                    self.items = markdown::parse(&contents).collect();
                }
                Task::none()
            }
            Message::LinkClicked(link) => {
                let _ = open::that_in_background(link.to_string());
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Top Content
        let menu_bar = row![horizontal_space(), text("")];

        // Main Content
        let text_editor_input = text_editor(&self.content)
            .height(Length::Fill)
            .on_action(Message::Edit)
            .style(text_editor_style);

        let preview = markdown(
            &self.items,
            markdown::Settings::default(),
            markdown::Style::from_palette(self.theme.palette()),
        )
        .map(Message::LinkClicked);

        let main = row![
            text_editor_input,
            scrollable(preview).spacing(10).height(Length::Fill)
        ]
        .spacing(10);

        // Bottom Content
        let position = {
            let (ln, col) = self.content.cursor_position();

            text(format!("Ln {}, Col {}", ln + 1, col + 1))
        };

        let path_text = match &self.file {
            Some(path) => format!("{}", path.display()),
            None => String::new(),
        };

        let filename = text(path_text);

        let status_bar = row![position, horizontal_space(), filename];

        // App Display
        container(column![menu_bar, main, status_bar])
            .padding(Padding::from([5, 5]))
            .style(editor_style)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

// In any case we can show a msg to the user
#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}
