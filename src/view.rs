use std::cmp;
use std::io::{stdout, Write};

use crossterm::terminal::ClearType;
use crossterm::{cursor, execute, queue, style, terminal};

use crate::status::StatusInfo;
use crate::{CursorController, EditorOutput, TAB_SIZE, VERSION};

/// 编辑器内容显示器
pub struct EditorView {
    // 终端窗口大小
    win_size: (usize, usize),
    // 终端窗口最大文本行数
    win_max_rows: usize,
    // 编辑器输出
    editor_output: EditorOutput,
    // 文本行
    edit_rows: Vec<EditRow>,
}

impl EditorView {
    // 重置并且隐藏光标
    fn reset_and_hide_cursor(&mut self) {
        queue!(self.editor_output, cursor::Hide, cursor::MoveTo(0, 0)).unwrap();
    }

    // 移动光标到指定位置
    fn move_cursor(&mut self, cc: &mut CursorController) {
        let cursor_x = cc.get_cursor().get_x() - cc.get_columns_offset();
        let cursor_y = cc.get_cursor().get_y() - cc.get_rows_offset();
        queue!(
            self.editor_output,
            cursor::MoveTo(cursor_x as u16, cursor_y as u16),
            cursor::Show
        )
        .unwrap();
    }

    // 绘制 banner
    fn draw_banner(&mut self) {
        let screen_columns = self.win_size.0;
        let mut welcome_text = format!("z-editor --- version: {}", VERSION);
        if welcome_text.len() > screen_columns {
            welcome_text.truncate(screen_columns);
        }
        let mut padding = (screen_columns - welcome_text.len()) / 2;
        if padding != 0 {
            self.editor_output.push('~');
            padding -= 1;
        }
        (0..padding).for_each(|_| self.editor_output.push(' '));
        self.editor_output.push_str(&welcome_text);
    }

    // 绘制文本
    fn draw_text(&mut self, cc: &mut CursorController, view_rows: usize) {
        let screen_columns = self.win_size.0;
        let text = &self.edit_rows[view_rows].rendered_content;
        let column_offset = cc.get_columns_offset();
        let mut len = cmp::min(text.len().saturating_sub(column_offset), screen_columns);
        // Note: 修复中文字符串截取问题
        while !text.is_char_boundary(len) {
            len -= 1;
        }
        let start = if len == 0 { 0 } else { column_offset };
        self.editor_output
            .push_str(&self.edit_rows[view_rows].rendered_content[start..start + len]);
    }

    // 绘制屏幕所有行
    fn draw_text_rows(&mut self, cc: &mut CursorController) {
        // 窗口留下两行用于打印其他信息
        let max_text_rows = self.win_size.1.saturating_sub(2);
        for i in 0..max_text_rows {
            let view_rows = i + cc.get_rows_offset();
            if view_rows >= self.edit_rows.len() {
                // 超出实际文本内容外的行
                self.editor_output.push('~');
            } else {
                self.draw_text(cc, view_rows);
            }
            queue!(self.editor_output, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            self.editor_output.push_str("\r\n");
        }
    }

    // 绘制状态栏
    fn draw_status_bar(&mut self, cc: &mut CursorController, file_name: &str) {
        self.editor_output
            .push_str(&style::Attribute::Reverse.to_string());
        let info = format!("{} -- {} lines", file_name, self.number_of_rows());
        let info_len = cmp::min(info.len(), self.win_size.0);
        let line_info = format!("{}/{}", cc.get_cursor().get_y() + 1, self.number_of_rows());
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

    // 绘制消息栏
    fn draw_message_bar(&mut self, status_info: &StatusInfo) {
        queue!(self.editor_output, terminal::Clear(ClearType::UntilNewLine)).unwrap();
        let msg = status_info.get_message();
        self.editor_output
            .push_str(&msg[..cmp::min(self.win_size.0, msg.len())]);
    }

    /// 创建编辑器内容显示器
    pub fn new(content: Vec<String>) -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        let win_max_rows = win_size.1.saturating_sub(2);
        info!("创建编辑视图，窗口大小为：{:?}", win_size);
        Self {
            win_size,
            win_max_rows,
            editor_output: EditorOutput::new(),
            edit_rows: content.into_iter().map(|it| EditRow::new(it)).collect(),
        }
    }

    /// 清除当前终端所有内容
    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }

    /// 刷新屏幕
    pub fn refresh_screen(&mut self, cc: &mut CursorController, status_info: &StatusInfo) {
        self.reset_and_hide_cursor();
        self.draw_text_rows(cc);
        self.draw_status_bar(cc, status_info.file_name_or_default());
        self.draw_message_bar(status_info);
        self.move_cursor(cc);
        self.editor_output.flush().unwrap();
    }

    /// 获取窗口大小
    pub fn get_win_size(&self) -> (usize, usize) {
        self.win_size
    }

    pub fn get_win_max_rows(&self) -> usize {
        self.win_max_rows
    }

    /// 获取文本内容总行数
    pub fn number_of_rows(&self) -> usize {
        self.edit_rows.len()
    }

    /// 获取指定行的渲染内容
    pub fn rendered_content_of_row(&self, i: usize) -> &str {
        self.edit_rows[i].get_rendered_content()
    }

    /// 获取指定行的原始内容
    pub fn raw_content_of_row(&self, i: usize) -> &str {
        self.edit_rows[i].get_raw_content()
    }

    /// 获取指定行
    pub fn get_edit_row(&self, i: usize) -> &EditRow {
        &self.edit_rows[i]
    }
}

impl Drop for EditorView {
    fn drop(&mut self) {
        EditorView::clear_screen().expect("Error");
    }
}

/// 编辑行
pub struct EditRow {
    // 原内容
    raw_content: String,
    // 渲染内容
    rendered_content: String,
}

impl EditRow {
    /// 创建编辑行
    pub fn new(raw_content: String) -> Self {
        let mut index = 0;
        let capacity = raw_content
            .chars()
            .fold(0, |acc, next| acc + if next == '\t' { TAB_SIZE } else { 1 });
        let mut rendered_content = String::with_capacity(capacity);
        raw_content.chars().for_each(|c| {
            index += 1;
            if c == '\t' {
                rendered_content.push(' ');
                while index % TAB_SIZE != 0 {
                    rendered_content.push(' ');
                    index += 1;
                }
            } else {
                rendered_content.push(c);
            }
        });
        Self {
            raw_content,
            rendered_content,
        }
    }

    /// 获取原内容
    pub fn get_raw_content(&self) -> &str {
        self.raw_content.as_str()
    }

    /// 获取渲染内容
    pub fn get_rendered_content(&self) -> &str {
        self.rendered_content.as_str()
    }
}
