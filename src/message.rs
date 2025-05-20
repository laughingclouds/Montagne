use std::path::PathBuf;
use std::sync::Arc;

use iced::{
    widget::{markdown, text_editor},
    window,
};

use crate::Mode;
use crate::montagne_file_io::Error;

// define messages (interactions of the application)
#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    LinkClicked(markdown::Url),
    NewFile,
    OpenFile,
    FileOpened(Result<(PathBuf, Arc<String>), Error>),
    SaveFile,
    FileSaved(Result<PathBuf, Error>),
    SetMode(Mode),
    TogglerToggled,
    WindowEvent(window::Event),
    CloseApp,
    CloseExitModal,
    FileModified, // file stored in storage has changed
    // user should either reload file or keep current changes (if is_dirty otherwise reload automatically)
    /// Load (reload) the active file.
    LoadFile,
}
