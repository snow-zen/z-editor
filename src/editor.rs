use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{cmp, env, fs};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{event, terminal};

use crate::log::EditLog;
use crate::status::StatusInfo;
use crate::view::EditorView;
use crate::CursorController;

/// 编辑器
pub struct Editor {
    // 编辑视图
    editor_view: EditorView,
    // 光标控制器
    cursor_controller: CursorController,
    // 窗口大小
    win_size: (usize, usize),
    // 状态信息
    status_info: StatusInfo,
    // 编辑日志
    edit_log: EditLog,
}

impl Editor {
    fn empty(win_size: (usize, usize), initial_message: String) -> Self {
        Self {
            win_size,
            editor_view: EditorView::new(Vec::new(), win_size),
            cursor_controller: CursorController::new(win_size),
            status_info: StatusInfo::new(None, 0, initial_message),
            edit_log: EditLog::new(),
        }
    }

    fn from_file(file: &Path, win_size: (usize, usize), initial_message: String) -> Self {
        let content: Vec<String> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|it| String::from(it))
            .collect();
        let lines = content.len();
        Self {
            win_size,
            editor_view: EditorView::new(content, win_size),
            cursor_controller: CursorController::new(win_size),
            status_info: StatusInfo::new(Some(file.to_path_buf()), lines, initial_message),
            edit_log: EditLog::new(),
        }
    }

    /// 创建编辑器
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();

        let mut arg = env::args();
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        let initial_message = "HELP: Ctrl-Q = Quit.".into();
        match arg.nth(1) {
            None => Self::empty(win_size, initial_message),
            Some(file) => Self::from_file(file.as_ref(), win_size, initial_message),
        }
    }

    /// 运行编辑器
    pub fn run(&mut self) {
        loop {
            let file_name = self.status_info.file_name_or_default();
            self.cursor_controller.scroll(&self.editor_view);
            self.editor_view
                .refresh_screen(&mut self.cursor_controller, file_name);
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
            } => self
                .cursor_controller
                .move_cursor(direction, &self.editor_view),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(val, KeyCode::PageUp) {
                    self.cursor_controller.get_cursor().1 = self.cursor_controller.get_row_offset();
                } else {
                    self.cursor_controller.get_cursor().1 = cmp::min(
                        self.editor_view.get_win_size().1 + self.cursor_controller.get_row_offset()
                            - 1,
                        self.editor_view.number_of_rows(),
                    )
                }
                (0..self.editor_view.get_win_size().1).for_each(|_| {
                    self.cursor_controller.move_cursor(
                        if matches!(val, KeyCode::PageUp) {
                            KeyCode::Up
                        } else {
                            KeyCode::Down
                        },
                        &self.editor_view,
                    );
                })
            }
            _ => {}
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("无法关闭 Raw 模式");
    }
}
