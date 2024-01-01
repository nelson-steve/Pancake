#![warn(clippy::all, clippy::pedantic)]

use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

mod editor;

use editor::Editor;

fn main() {
    crossterm::terminal::enable_raw_mode().ok();

    let editor = Editor::default();
    editor.run();
}
