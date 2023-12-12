mod editor;

use editor::Editor;

fn main() {
    crossterm::terminal::enable_raw_mode().ok();

    let editor = Editor::default();
    editor.run();
}
