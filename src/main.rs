use crossterm::terminal;

use crate::cursor_controller::CursorController;
use crate::editor::Editor;
use crate::editor_content_display::EditorContentDisplay;
use crate::editor_output::EditorOutput;

mod cursor_controller;
mod editor;
mod editor_content_display;
mod editor_output;

/// 用于关闭程序时清理资源
struct CleaUp;

impl Drop for CleaUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("无法关闭 Raw 模式");
        EditorContentDisplay::clear_screen().expect("Error")
    }
}

/// 编辑器版本
const VERSION: &str = "0.0.1";

fn main() -> crossterm::Result<()> {
    let _clean_up = CleaUp;
    terminal::enable_raw_mode()?;
    let mut editor = Editor::new();
    editor.run();
    Ok(())
}
