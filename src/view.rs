use std::cmp;
use std::io::{stdout, Write};

use crossterm::style::style;
use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, style, terminal};

use crate::{CursorController, EditorOutput, StatusMessage, TAB_STOP, VERSION};

/// 编辑器内容显示器
pub struct EditorView {
    // 终端窗口大小
    win_size: (usize, usize),
    // 编辑器输出
    editor_output: EditorOutput,
    // 文本行
    rows: Vec<Row>,
    // 状态信息
    status_message: StatusMessage,
}

impl EditorView {
    /// 创建编辑器内容显示器
    pub fn new(content: Vec<String>, win_size: (usize, usize)) -> Self {
        Self {
            win_size,
            editor_output: EditorOutput::new(),
            rows: content.into_iter().map(|it| Row::new(it)).collect(),
            status_message: StatusMessage::new("HELP: Ctrl-Q = Quit.".into()),
        }
    }

    /// 清除当前终端所有内容
    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /// 刷新屏幕
    pub fn refresh_screen(&mut self, cc: &mut CursorController, file_name: &str) {
        queue!(self.editor_output, cursor::Hide, cursor::MoveTo(0, 0)).unwrap();
        self.draw_rows(cc);
        self.draw_status_bar(cc, file_name);
        self.draw_message_bar();
        let cursor_x = cc.get_render_x() - cc.get_column_offset();
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
            if file_rows >= self.rows.len() {
                if self.rows.len() == 0 && i == screen_rows / 3 {
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
                let text = &self.rows[file_rows].render;
                let column_offset = cc.get_column_offset();
                let mut len = cmp::min(text.len().saturating_sub(column_offset), screen_columns);
                // Note: 修复中文字符串截取问题
                while !text.is_char_boundary(len) {
                    len -= 1;
                }
                let start = if len == 0 { 0 } else { column_offset };
                self.editor_output
                    .push_str(&self.rows[file_rows].render[start..start + len]);
            }
            queue!(self.editor_output, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            self.editor_output.push_str("\r\n");
            // stdout().flush().unwrap();
        }
    }

    pub fn draw_status_bar(&mut self, cc: &mut CursorController, file_name: &str) {
        self.editor_output
            .push_str(&style::Attribute::Reverse.to_string());
        let info = format!("{} -- {} lines", file_name, self.number_of_rows());
        let info_len = cmp::min(info.len(), self.win_size.0);
        let line_info = format!("{}/{}", cc.get_cursor().1 + 1, self.number_of_rows());
        self.editor_output.push_str(&info[..info_len]);
        for i in info_len..self.win_size.0 {
            if self.win_size.0 - i == line_info.len() {
                self.editor_output.push_str(&line_info);
                break;
            } else {
                self.editor_output.push(' ');
            }
        }
        self.editor_output
            .push_str(&style::Attribute::Reset.to_string());
        self.editor_output.push_str("\r\n");
    }

    pub fn draw_message_bar(&mut self) {
        queue!(self.editor_output, terminal::Clear(ClearType::UntilNewLine)).unwrap();
        if let Some(msg) = self.status_message.message() {
            self.editor_output.push_str(msg);
            self.editor_output
                .push_str(&msg[..cmp::min(self.win_size.0, msg.len())]);
        }
    }

    pub fn get_win_size(&self) -> (usize, usize) {
        self.win_size
    }

    pub fn number_of_rows(&self) -> usize {
        self.rows.len()
    }

    pub fn get_render_row(&self, i: usize) -> &str {
        self.rows[i].render.as_str()
    }

    pub fn get_row(&self, i: usize) -> &Row {
        &self.rows[i]
    }
}

impl Drop for EditorView {
    fn drop(&mut self) {
        EditorView::clear_screen().expect("Error");
    }
}

pub struct Row {
    row_content: String,
    render: String,
}

impl Row {
    fn new(row_content: String) -> Self {
        let mut index = 0;
        let capacity = row_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_STOP } else { 1 });
        let mut render = String::with_capacity(capacity);
        row_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                render.push(' ');
                while index % TAB_STOP != 0 {
                    render.push(' ');
                    index += 1;
                }
            } else {
                render.push(c);
            }
        });
        Self {
            row_content,
            render,
        }
    }

    pub fn get_row_content(&self) -> &str {
        self.row_content.as_str()
    }
}
