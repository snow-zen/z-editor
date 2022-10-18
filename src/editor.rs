use std::path::{Path, PathBuf};
use std::time::Duration;
use std::{cmp, env, fs};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{event, terminal};

use crate::status::StatusInfo;
use crate::view::EditView;
use crate::{CursorController, EditorContentDisplay};

/// 编辑器
pub struct Editor {
    // 编辑器内容显示器
    editor_content_display: EditorContentDisplay,
    // 光标控制器
    cursor_controller: CursorController,
    // 文件路径
    file_name: Option<PathBuf>,
    // 窗口大小
    win_size: (usize, usize),
    // 状态信息
    status_info: StatusInfo,
    // 编辑视图
    edit_view: EditView,
}

impl Editor {
    /// 创建编辑器
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();

        let mut arg = env::args();
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        let initial_message = "HELP: Ctrl-Q = Quit.".into();
        match arg.nth(1) {
            None => Self {
                editor_content_display: EditorContentDisplay::new(Vec::new(), win_size),
                cursor_controller: CursorController::new(win_size),
                file_name: None,
                win_size,
                status_info: StatusInfo::new(None, 0, initial_message),
                edit_view: EditView::new(),
            },
            Some(file) => {
                let content: Vec<String> =
                    fs::read_to_string(<String as AsRef<Path>>::as_ref(&file))
                        .unwrap()
                        .lines()
                        .map(|it| String::from(it))
                        .collect();
                let lines = content.len();
                Self {
                    editor_content_display: EditorContentDisplay::new(content, win_size),
                    cursor_controller: CursorController::new(win_size),
                    file_name: Some(file.clone().into()),
                    win_size,
                    status_info: StatusInfo::new(Some(file.clone().into()), lines, initial_message),
                    edit_view: EditView::new(),
                }
            }
        }
    }

    /// 运行编辑器
    pub fn run(&mut self) {
        loop {
            let file_name = self
                .file_name
                .as_ref()
                .and_then(|path| path.file_name())
                .and_then(|name| name.to_str())
                .unwrap_or("[No Name]");
            self.cursor_controller.scroll(&self.editor_content_display);
            self.editor_content_display
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
                .move_cursor(direction, &self.editor_content_display),
            KeyEvent {
                code: val @ (KeyCode::PageUp | KeyCode::PageDown),
                modifiers: KeyModifiers::NONE,
            } => {
                if matches!(val, KeyCode::PageUp) {
                    self.cursor_controller.get_cursor().1 = self.cursor_controller.get_row_offset();
                } else {
                    self.cursor_controller.get_cursor().1 = cmp::min(
                        self.editor_content_display.get_win_size().1
                            + self.cursor_controller.get_row_offset()
                            - 1,
                        self.editor_content_display.number_of_rows(),
                    )
                }
                (0..self.editor_content_display.get_win_size().1).for_each(|_| {
                    self.cursor_controller.move_cursor(
                        if matches!(val, KeyCode::PageUp) {
                            KeyCode::Up
                        } else {
                            KeyCode::Down
                        },
                        &self.editor_content_display,
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
