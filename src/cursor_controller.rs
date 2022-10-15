use std::cmp;

use crate::EditorContentDisplay;
use crossterm::event::KeyCode;

/// 光标控制器
pub struct CursorController {
    // 光标列偏移量
    cursor_x: usize,
    // 光标行偏移量
    cursor_y: usize,
    // 窗口最大行数
    screen_rows: usize,
    // 窗口最大列数
    screen_columns: usize,
    // 行偏移量
    row_offset: usize,
    // 列偏移量
    column_offset: usize,
}

impl CursorController {
    /// 创建初始光标控制器
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_rows: win_size.1,
            screen_columns: win_size.0,
            row_offset: 0,
            column_offset: 0,
        }
    }

    /// 获取当前光标位置元组，索引 0 为列，索引 1 为行
    pub fn get_cursor(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    /// 获取当前行偏移量
    pub fn get_row_offset(&self) -> usize {
        self.row_offset
    }

    pub fn get_column_offset(&self) -> usize {
        self.column_offset
    }

    /// 屏幕滚动
    pub fn scroll(&mut self) {
        // 行偏移量变化
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
        // 列偏移量变化
        self.column_offset = cmp::min(self.column_offset, self.cursor_x);
        if self.cursor_x >= self.column_offset + self.screen_columns {
            self.column_offset = self.cursor_x - self.screen_columns + 1;
        }
    }

    /// 移动光标
    pub fn move_cursor(&mut self, direction: KeyCode, ecd: &EditorContentDisplay) {
        let number_of_rows = ecd.number_of_rows();
        // todo 移动光标支持中文
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            }
            KeyCode::Right => {
                if self.cursor_y < number_of_rows
                    && self.cursor_x < ecd.get_row(self.cursor_y).len()
                {
                    self.cursor_x += 1;
                }
            }
            KeyCode::Down => {
                if self.cursor_y < number_of_rows {
                    self.cursor_y += 1;
                }
            }
            KeyCode::Home => {
                self.cursor_x = 0;
            }
            KeyCode::End => {
                self.cursor_x = self.screen_columns - 1;
            }
            _ => unimplemented!(),
        }
        let row_len = if self.cursor_y < number_of_rows {
            ecd.get_row(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, row_len);
    }
}
