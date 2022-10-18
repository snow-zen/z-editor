use crate::cursor_controller::CursorController;
use crate::editor::Editor;
use crate::view::EditorView;
use crate::editor_output::EditorOutput;
use crate::status_message::StatusMessage;

mod cursor_controller;
mod editor;
mod view;
mod editor_output;
mod status;
mod status_message;
mod log;

/// 编辑器版本
const VERSION: &str = "0.0.1";
/// 制表符大小
const TAB_STOP: usize = 8;

fn main() -> crossterm::Result<()> {
    let mut editor = Editor::new();
    editor.run();
    Ok(())
}
