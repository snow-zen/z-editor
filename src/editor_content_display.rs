use std::cmp;
use std::io::{stdout, Write};

use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, terminal};

use crate::{CursorController, EditorOutput, VERSION};

/// 编辑器内容显示器
pub struct EditorContentDisplay {
    // 终端窗口大小
    win_size: (usize, usize),
    // 编辑器输出
    editor_output: EditorOutput,
    // 文本内容
    content: Vec<String>,
}

impl EditorContentDisplay {
    /// 创建编辑器内容显示器
    pub fn new(content: Vec<String>, win_size: (usize, usize)) -> Self {
        Self {
            win_size,
            editor_output: EditorOutput::new(),
            content,
        }
    }

    /// 清除当前终端所有内容
    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /// 刷新屏幕
    pub fn refresh_screen(&mut self, cc: &mut CursorController) {
        cc.scroll();
        queue!(self.editor_output, cursor::Hide, cursor::MoveTo(0, 0)).unwrap();
        self.draw_rows(cc);
        let cursor_x = cc.get_cursor().0 - cc.get_column_offset();
        let cursor_y = cc.get_cursor().1 - cc.get_row_offset();
        queue!(
            self.editor_output,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )
        .unwrap();
        self.editor_output.flush().unwrap();
    }

    pub fn draw_rows(&mut self, cc: &mut CursorController) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            let file_rows = i + cc.get_row_offset();
            if file_rows >= self.content.len() {
                if self.content.len() == 0 && i == screen_rows / 3 {
                    let mut welcome = format!("z-editor --- version: {}", VERSION);
                    if welcome.len() > screen_columns {
                        welcome.truncate(screen_columns);
                    }
                    let mut padding = (screen_columns - welcome.len()) / 2;
                    if padding != 0 {
                        self.editor_output.push('~');
                        padding -= 1;
                    }
                    (0..padding).for_each(|_| self.editor_output.push(' '));
                    self.editor_output.push_str(&welcome);
                } else {
                    self.editor_output.push('~');
                }
            } else {
                let text = &self.content[file_rows];
                let column_offset = cc.get_column_offset();
                let mut len = cmp::min(text.len().saturating_sub(column_offset), screen_columns);
                // Note: 修复中文字符串截取问题
                while !text.is_char_boundary(len) {
                    len -= 1;
                }
                let start = if len == 0 { 0 } else { column_offset };
                self.editor_output
                    .push_str(&self.content[file_rows][start..start + len]);
            }
            queue!(self.editor_output, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < screen_rows - 1 {
                self.editor_output.push_str("\r\n");
            }
            stdout().flush().unwrap();
        }
    }

    pub fn get_win_size(&self) -> (usize, usize) {
        self.win_size
    }

    pub fn number_of_rows(&self) -> usize {
        self.content.len()
    }

    pub fn get_row(&self, i: usize) -> &str {
        self.content[i].as_str()
    }
}
