// #![windows_subsystem = "windows"]
use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{
    button, column, container, horizontal_space, markdown, row, scrollable, text, text_editor,
    toggler, tooltip,
};
use iced::{Element, Length};
use iced::{Font, Padding, Task, Theme};

mod montagne_theme;
use montagne_theme::{editor_style, new_icon, open_icon, save_icon, text_editor_style};

mod montagne_file_io;
use montagne_file_io::{Error, open_file, save_file};

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
    is_splitview: bool,

    application_msg: String,
}

#[derive(Debug, Clone)]
enum Mode {
    Write,
    Preview,
    Split,
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
    SetMode(Mode),
    TogglerToggled(bool),
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
                application_mode: Mode::Write,
                is_splitview: false,
                application_msg: String::from("Welcome to Montagne."),
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
            Message::FileOpened(Err(err)) | Message::FileSaved(Err(err)) => {
                match err {
                    Error::DialogClosed => self.application_msg = "Dialogue closed".to_string(),
                    Error::IoError(kind) => self.application_msg = format!("I/O Error {}", kind),
                }
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
            Message::SetMode(mode) => {
                if matches!(mode, Mode::Preview | Mode::Split) {
                    self.items = markdown::parse(&self.content.text()).collect();
                }

                self.application_mode = mode;

                Task::none()
            }
            Message::TogglerToggled(is_splitview) => {
                self.is_splitview = is_splitview;

                if is_splitview {
                    Task::done(Message::SetMode(Mode::Split))
                } else {
                    Task::done(Message::SetMode(Mode::Write))
                }
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        // Top Content
        let header = {
            let mut menu_bar = row![
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
                ),
                horizontal_space()
            ];

            menu_bar = match &self.application_mode {
                Mode::Write => {
                    menu_bar.push(button("Preview").on_press(Message::SetMode(Mode::Preview)))
                }
                Mode::Preview => {
                    menu_bar.push(button("Write").on_press(Message::SetMode(Mode::Write)))
                }
                Mode::Split => menu_bar,
            };

            menu_bar = menu_bar.push(
                toggler(self.is_splitview)
                    .label("Split")
                    .on_toggle(Message::TogglerToggled),
            );

            menu_bar
        };

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
                Mode::Write => row![text_editor_input],
                Mode::Preview => row![preview],
                Mode::Split => row![text_editor_input, preview],
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
        container(column![header, main, status_bar])
            .padding(Padding::from([5, 5]))
            .style(editor_style)
            .into()
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
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
