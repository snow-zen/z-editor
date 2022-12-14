use std::path::Path;
use std::time::Duration;
use std::{cmp, env, fs};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::{event, terminal};

use crate::edit_log::EditLog;
use crate::status::StatusInfo;
use crate::view::EditorView;
use crate::CursorController;

/// 编辑器
pub struct Editor {
    // 编辑视图
    editor_view: EditorView,
    // 光标控制器
    cursor_controller: CursorController,
    // 状态信息
    status_info: StatusInfo,
    // 编辑日志
    edit_log: EditLog,
}

impl Editor {
    // 创建空内容的编辑器
    fn empty(initial_message: String) -> Self {
        Self {
            editor_view: EditorView::new(Vec::new()),
            cursor_controller: CursorController::new(),
            status_info: StatusInfo::new(None, 0, initial_message),
            edit_log: EditLog::new(),
        }
    }

    // 根据指定文件创建编辑器
    fn from_file(file: &Path, initial_message: String) -> Self {
        let content: Vec<String> = fs::read_to_string(file)
            .unwrap()
            .lines()
            .map(|it| String::from(it))
            .collect();
        let lines = content.len();
        info!("读取文件：{:?}，总行数：{}", file, lines);
        Self {
            editor_view: EditorView::new(content),
            cursor_controller: CursorController::new(),
            status_info: StatusInfo::new(Some(file.to_path_buf()), lines, initial_message),
            edit_log: EditLog::new(),
        }
    }

    /// 创建编辑器
    pub fn new() -> Self {
        terminal::enable_raw_mode().unwrap();

        let mut arg = env::args();
        let initial_message = "HELP: Ctrl-Q = Quit.".into();
        info!("启动编辑器，启动参数：{:?}", arg);

        match arg.nth(1) {
            None => Self::empty(initial_message),
            Some(file) => Self::from_file(file.as_ref(), initial_message),
        }
    }

    /// 运行编辑器
    pub fn run(&mut self) {
        loop {
            self.cursor_controller.scroll(&self.editor_view);
            self.editor_view
                .refresh_screen(&mut self.cursor_controller, &self.status_info);
            let mut exit_flag = false;
            if self.is_event_available().unwrap() {
                if let Event::Key(event) = event::read().unwrap() {
                    debug!("检测到输入事件：{:?}", event);
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
                    self.cursor_controller.move_page_up(&self.editor_view);
                } else {
                    self.cursor_controller.move_page_down(&self.editor_view);
                }
            }
            _ => {}
        }
    }
}

impl Drop for Editor {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("无法关闭 Raw 模式");
        info!("关闭编辑器")
    }
}
