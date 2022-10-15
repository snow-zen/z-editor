use std::path::Path;
use std::time::Duration;
use std::{env, fs};

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use crate::EditorContentDisplay;

/// 编辑器
pub struct Editor {
    // 编辑器内容显示器
    editor_content_display: EditorContentDisplay,
}

impl Editor {
    /// 创建编辑器
    pub fn new() -> Self {
        let mut arg = env::args();
        let contents = match arg.nth(1) {
            None => Vec::new(),
            Some(file) => fs::read_to_string(<String as AsRef<Path>>::as_ref(&file))
                .unwrap()
                .lines()
                .map(|it| String::from(it))
                .collect(),
        };
        Self {
            editor_content_display: EditorContentDisplay::new(contents),
        }
    }

    /// 运行编辑器
    pub fn run(&mut self) {
        loop {
            self.editor_content_display.refresh_screen();
            let mut exit_flag = false;
            if self.is_event_available().unwrap() {
                if let Event::Key(event) = event::read().unwrap() {
                    self.process_key(event, &mut exit_flag);
                }
            }

            if exit_flag {
                break;
            }
        }
    }

    /// 判断是否有按键事件可用
    fn is_event_available(&self) -> crossterm::Result<bool> {
        event::poll(Duration::from_millis(500))
    }

    /// 处理按键事件
    fn process_key(&mut self, event: KeyEvent, exit_flag: &mut bool) {
        match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
            } => *exit_flag = true,
            KeyEvent {
                code:
                    direction @ (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::Home
                    | KeyCode::End),
                modifiers: KeyModifiers::NONE,
            } => self.editor_content_display.move_cursor(direction),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => (0..self.editor_content_display.get_win_size().1).for_each(|_| {
                self.editor_content_display
                    .move_cursor(if matches!(val, KeyCode::PageUp) {
                        KeyCode::Up
                    } else {
                        KeyCode::Down
                    });
            }),
            _ => {}
        }
    }
}
