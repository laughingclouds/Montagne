// #![windows_subsystem = "windows"]

mod montagne_theme;

use montagne_theme::{editor_style, new_icon, open_icon, save_icon, text_editor_style};

use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use iced::widget::{
    button, column, container, horizontal_space, markdown, row, scrollable, text, text_editor,
    tooltip,
};
use iced::{Element, Length};
use iced::{Font, Padding, Task, Theme};

fn main() -> iced::Result {
    iced::application("Montagne", Montagne::update, Montagne::view)
        .centered()
        .theme(Montagne::theme)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .default_font(Font::MONOSPACE)
        .run_with(Montagne::new)
}

// define state
struct Montagne {
    content: text_editor::Content,
    items: Vec<markdown::Item>,
    file: Option<PathBuf>,

    theme: Theme,

    is_loading: bool,
    is_dirty: bool,

    application_mode: Mode,
}

#[derive(Debug, Clone)]
enum Mode {
    WriteMode,
    PreviewMode,
    SplitMode,
}

// define messages (interactions of the application)
#[derive(Debug, Clone)]
enum Message {
    Edit(text_editor::Action),
    LinkClicked(markdown::Url),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    ApplicationModeChanged(Mode),
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
                is_loading: false,
                is_dirty: false,
                application_mode: Mode::WriteMode,
            },
            // change later to reload tabs (or previously opened editors)
            // Task::batch([
            //     Task::perform(
            //         load_file(format!("{}\\target\\README.md", env!("CARGO_MANIFEST_DIR"))),
            //         Message::FileOpened,
            //     ),
            //     widget::focus_next(),
            // ]),
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit(action) => {
                // let is_edit = action.is_edit();

                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                // if is_edit {
                //     self.items = markdown::parse(&self.content.text()).collect();
                // }

                Task::none()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                    // optionally check what mode the file is opened with
                    // self.items = markdown::parse(&self.content.text()).collect();
                }

                Task::none()
            }
            Message::OpenFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(open_file(), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                self.is_loading = false;
                self.is_dirty = false;

                if let Ok((path, content)) = result {
                    self.content = text_editor::Content::with_text(&content);
                    // optionally check what mode the file is opened with
                    // self.items = markdown::parse(&content).collect();
                    self.file = Some(path);
                }

                Task::none()
            }
            Message::SaveFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(
                        save_file(self.file.clone(), self.content.text()),
                        Message::FileSaved,
                    )
                }
            }
            Message::FileSaved(result) => {
                self.is_loading = false;

                if let Ok(path) = result {
                    self.file = Some(path);
                    self.is_dirty = false;
                }

                Task::none()
            }
            Message::LinkClicked(link) => {
                let _ = open::that_in_background(link.to_string());
                Task::none()
            }
            Message::ApplicationModeChanged(mode) => {
                if matches!(mode, Mode::PreviewMode | Mode::SplitMode) {
                    self.items = markdown::parse(&self.content.text()).collect()
                }

                self.application_mode = mode;

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Top Content
        let menu_bar = row![
            action(new_icon(), "New file", Some(Message::NewFile)),
            action(
                open_icon(),
                "Open file",
                (!self.is_loading).then_some(Message::OpenFile)
            ),
            action(
                save_icon(),
                "Save file",
                (self.is_dirty).then_some(Message::SaveFile)
            )
        ];

        // Main Content
        let main = {
            let text_editor_input = text_editor(&self.content)
                .height(Length::Fill)
                .on_action(Message::Edit)
                .style(text_editor_style);

            let preview = scrollable(
                markdown(
                    &self.items,
                    markdown::Settings::default(),
                    markdown::Style::from_palette(self.theme.palette()),
                )
                .map(Message::LinkClicked),
            )
            .spacing(10)
            .height(Length::Fill);

            let main_content = match &self.application_mode {
                Mode::WriteMode => row![text_editor_input],
                Mode::PreviewMode => row![preview],
                Mode::SplitMode => row![text_editor_input, preview],
            };

            main_content.spacing(10)
        };

        // Bottom Content
        let status_bar = {
            let position = {
                let (ln, col) = self.content.cursor_position();

                text(format!("Ln {}, Col {}", ln + 1, col + 1))
            };

            let path_text = match &self.file {
                Some(path) => {
                    let path = path.display().to_string();

                    // since our file path is on the right end we can
                    // afford to have more space
                    if path.len() > 80 {
                        format!("...{}", &path[path.len() - 40..])
                    } else {
                        path
                    }
                }
                None => String::from("New file"),
            };

            let filename = text(path_text);

            row![position, horizontal_space(), filename]
        };

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

// Asynchronous flow for opening a file picker and then calling load_file()
async fn open_file() -> Result<(PathBuf, Arc<String>), Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .set_title("Open a markdown file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;

    load_file(picked_file).await
}

// Asynchronously load a file given its PathBuffer
async fn load_file(path: impl Into<PathBuf>) -> Result<(PathBuf, Arc<String>), Error> {
    let path = path.into();

    let contents = tokio::fs::read_to_string(&path)
        .await
        .map(Arc::new)
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok((path, contents))
}

async fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::AsyncFileDialog::new()
            .save_file()
            .await
            .as_ref()
            .map(rfd::FileHandle::path)
            .map(Path::to_owned)
            .ok_or(Error::DialogClosed)?
    };

    tokio::fs::write(&path, contents)
        .await
        .map_err(|error| Error::IoError(error.kind()))?;

    Ok(path)
}

// A wrapper for any on_press events on buttons.
fn action<'a, Message: Clone + 'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Option<Message>,
) -> Element<'a, Message> {
    let action = button(container(content).center_x(30));

    if let Some(on_press) = on_press {
        tooltip(
            action.on_press(on_press),
            label,
            tooltip::Position::FollowCursor,
        )
        .style(container::rounded_box)
        .into()
    } else {
        action.style(button::secondary).into()
    }
}
