// #![windows_subsystem = "windows"]
use std::path::PathBuf;
use std::sync::Arc;

use iced::widget::{
    self, button, center, column, container, horizontal_space, markdown, opaque, row, scrollable,
    stack, text, text_editor, toggler, tooltip,
};
use iced::{Element, Length, Subscription, window};
use iced::{Padding, Task, Theme, highlighter};

mod montagne_theme;
use montagne_theme::{
    editor_style, exit_modal_style, new_icon, open_icon, preview_scrollable_style, save_icon,
};

mod montagne_file_io;
use montagne_file_io::{Error, open_file, save_file};

fn main() -> iced::Result {
    iced::application("Montagne", Montagne::update, Montagne::view)
        .subscription(Montagne::subscription)
        .exit_on_close_request(false)
        .centered()
        // .default_font(Font::MONOSPACE)
        .font(include_bytes!("../fonts/icons.ttf").as_slice())
        .theme(Montagne::theme)
        .run_with(Montagne::new)
}

#[derive(Debug, Clone)]
enum Mode {
    Write,
    Preview,
    Split,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Mode::Write => write!(f, "Write"),
            Mode::Preview => write!(f, "Preview"),
            Mode::Split => write!(f, "Split"),
        }
    }
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
    WindowEvent(window::Event),
    CloseApp,
    CloseExitModal,
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

    show_exit_modal: bool,
}

impl Montagne {
    fn new() -> (Self, Task<Message>) {
        let theme = Theme::KanagawaDragon;
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
                show_exit_modal: false,
            },
            // change later to reload tabs (or previously opened editors)
            Task::none(),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowEvent(window::Event::CloseRequested) => {
                if self.is_dirty {
                    self.application_msg = "Close Requested".to_string();
                    self.show_exit_modal = true;
                    widget::focus_next()
                } else {
                    Task::done(Message::CloseApp)
                }
            }
            Message::WindowEvent(_) => Task::none(),
            Message::CloseApp => window::get_latest().and_then(window::close),
            Message::CloseExitModal => {
                self.application_msg = "Request Cancelled".to_string();
                self.show_exit_modal = false;
                Task::none()
            }
            Message::Edit(action) => {
                // let is_edit = action.is_edit();

                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                if self.is_splitview {
                    self.items = markdown::parse(&self.content.text()).collect();
                }

                Task::none()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.file = None;
                    self.content = text_editor::Content::new();
                    // optionally check what mode the file is opened with
                    if matches!(&self.application_mode, Mode::Preview | Mode::Split) {
                        self.items = markdown::parse(&self.content.text()).collect();
                    }
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

                match result {
                    Err(Error::DialogClosed) => {
                        self.application_msg = "Dialogue closed".to_string();
                    }
                    Err(Error::IoError(kind)) => {
                        self.application_msg = format!("I/O Error {}", kind);
                        eprint!("{}", kind)
                    }
                    Ok((path, content)) => {
                        self.content = text_editor::Content::with_text(&content);
                        self.file = Some(path);
                        self.application_msg = "File Opened".to_string();

                        if matches!(&self.application_mode, Mode::Preview | Mode::Split) {
                            self.items = markdown::parse(&self.content.text()).collect();
                        }
                    }
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

                match result {
                    Err(Error::DialogClosed) => {
                        self.application_msg = "Dialogue closed".to_string();
                    }
                    Err(Error::IoError(kind)) => {
                        self.application_msg = format!("I/O Error {}", kind)
                    }
                    Ok(path) => {
                        self.file = Some(path);
                        self.is_dirty = false;
                        self.application_msg = "File Saved".to_string();
                    }
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

                self.application_msg = format!("{} mode", mode);

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

    fn subscription(&self) -> Subscription<Message> {
        window::events().map(|(_id, event)| Message::WindowEvent(event))
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
                .highlight("md", highlighter::Theme::InspiredGitHub)
                .on_action(Message::Edit);

            let mut preview = scrollable(
                markdown(
                    &self.items,
                    markdown::Settings::default(),
                    markdown::Style::from_palette(Theme::TokyoNight.palette()),
                )
                .map(Message::LinkClicked),
            )
            .spacing(10)
            .height(Length::Fill);

            let main_content = match &self.application_mode {
                Mode::Write => row![text_editor_input],
                Mode::Preview => {
                    preview = preview.style(preview_scrollable_style);
                    row![center(
                        container(preview).width(Length::Shrink).max_width(800)
                    )]
                }
                // Mode::Preview => row![horizontal_space(), preview, horizontal_space()],
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
                Some(path) => path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| name.to_string())
                    .unwrap_or_else(|| "Unnamed file".to_string()),
                None => String::from("New file"),
            };

            let filename = text(path_text);

            row![
                position,
                horizontal_space(),
                text(&self.application_msg),
                horizontal_space(),
                filename
            ]
        };

        // App Display
        let app = container(column![header, main, status_bar])
            .padding(Padding::from([5, 5]))
            .style(editor_style);

        if self.show_exit_modal {
            stack![
                app,
                opaque(
                    center(opaque(column![
                        text("You have unsaved work. Close?"),
                        row![
                            button("Yes").on_press(Message::CloseApp),
                            button("No").on_press(Message::CloseExitModal),
                        ]
                    ]))
                    .style(exit_modal_style)
                ),
            ]
            .into()
        } else {
            app.into()
        }
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
        tooltip(action.on_press(on_press), label, tooltip::Position::Bottom)
            .style(container::rounded_box)
            .into()
    } else {
        action.style(button::secondary).into()
    }
}
