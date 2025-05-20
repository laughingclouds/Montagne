// #![windows_subsystem = "windows"]
use std::path::PathBuf;

use iced::{
    Alignment, Element, Length, Padding, Subscription, Task, Theme, highlighter,
    widget::{
        self, button, center, column, container, horizontal_space, markdown, row, scrollable, text,
        text_editor, toggler,
    },
    window,
};

mod message;
use message::Message;

mod custom_widget;
use custom_widget::{action, modal::{exit_modal, file_changed_modal}};

mod montagne_theme;
use montagne_theme::{editor_style, new_icon, open_icon, preview_scrollable_style, save_icon};

mod montagne_file_io;
use montagne_file_io::{Error, load_file, open_file, save_file};

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

// define state
struct Montagne {
    content: text_editor::Content,
    items: Vec<markdown::Item>,
    active_file: Option<PathBuf>,

    theme: Theme,

    is_loading: bool,
    is_dirty: bool,

    application_mode: Mode,
    application_msg: String,

    is_show_exit_modal: bool,
    is_show_file_changed_modal: bool,
}

impl Montagne {
    fn new() -> (Self, Task<Message>) {
        let theme = Theme::KanagawaDragon;
        (
            Self {
                content: text_editor::Content::new(),
                items: markdown::parse("").collect(),
                active_file: None,
                theme: theme,
                is_loading: false,
                is_dirty: false,
                application_mode: Mode::Write,
                application_msg: String::from("Welcome to Montagne."),
                is_show_exit_modal: false,
                is_show_file_changed_modal: false,
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
                    self.is_show_exit_modal = true;
                    widget::focus_next()
                } else {
                    Task::done(Message::CloseApp)
                }
            }
            Message::WindowEvent(_) => Task::none(),
            Message::CloseApp => window::get_latest().and_then(window::close),
            Message::CloseExitModal => {
                self.application_msg = "Request Cancelled".to_string();
                self.is_show_exit_modal = false;
                Task::none()
            }
            Message::Edit(action) => {
                if self.is_show_exit_modal {
                    return Task::none();
                }

                self.is_dirty = self.is_dirty || action.is_edit();

                self.content.perform(action);

                if matches!(self.application_mode, Mode::Split | Mode::Preview) {
                    self.items = markdown::parse(&self.content.text()).collect();
                }

                Task::none()
            }
            Message::FileModified => {
                if self.is_dirty {
                    self.is_show_file_changed_modal = true;
                    Task::none()
                } else {
                    self.load_active_file_or_set_error()
                }
            }
            Message::LoadFile => {
                self.is_loading = true;
                self.load_active_file_or_set_error()
            }
            Message::NewFile => {
                if !self.is_loading {
                    self.active_file = None;
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

                match result {
                    Err(Error::DialogClosed) => {
                        self.application_msg = "Dialogue closed".to_string();
                    }
                    Err(Error::IoError(kind)) => {
                        self.application_msg = format!("I/O Error {}", kind);
                        eprint!("{}", kind)
                    }
                    Ok((path, content)) => {
                        self.is_dirty = false;
                        self.content = text_editor::Content::with_text(&content);
                        self.active_file = Some(path);
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
                        save_file(self.active_file.clone(), self.content.text()),
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
                        self.active_file = Some(path);
                        self.is_dirty = false; // is_dirty becomes false only when we know it for sure
                        self.application_msg = "File Saved".to_string();

                        // also close the exit modal if we saved from there
                        self.is_show_exit_modal = false;
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
            Message::TogglerToggled => {
                if matches!(self.application_mode, Mode::Preview | Mode::Write) {
                    Task::done(Message::SetMode(Mode::Split))
                } else {
                    Task::done(Message::SetMode(Mode::Write))
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        let window_events = window::events().map(|(_id, event)| Message::WindowEvent(event));

        Subscription::batch([window_events])
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
            ]
            .align_y(Alignment::Center);

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
                toggler(matches!(self.application_mode, Mode::Split))
                    .label("Split")
                    .on_toggle(|_| Message::TogglerToggled),
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

            let path_text = match &self.active_file {
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

        if self.is_show_exit_modal {
            exit_modal(app)
        } else if self.is_show_file_changed_modal {
            match &self.active_file {
                Some(path) => file_changed_modal(app, path.clone()),
                None => app.into()
            }
        }
         else {
            app.into()
        }
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

impl Montagne {
    fn load_active_file_or_set_error(&mut self) -> Task<Message> {
        match &self.active_file {
            Some(path) => Task::perform(load_file(path.clone()), Message::FileOpened),
            None => {
                self.application_msg = "Error: No file path for active file.".to_string();
                Task::none()
            }
        }
    }
}
