use crate::cursor_controller::CursorController;
use crate::editor::Editor;
use crate::editor_output::EditorOutput;
use crate::view::EditorView;
use env_logger::{Builder, Target};
use log::LevelFilter;

mod cursor_controller;
mod edit_log;
mod editor;
mod editor_output;
mod status;
mod view;

#[macro_use]
extern crate log;

/// 编辑器版本
const VERSION: &str = "0.0.1";
/// 制表符大小
const TAB_SIZE: usize = 8;

fn main() -> crossterm::Result<()> {
    init_log();
    let mut editor = Editor::new();
    editor.run();
    Ok(())
}

// 初始化日志设置
fn init_log() {
    Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .target(Target::Stderr)
        .init();
}
