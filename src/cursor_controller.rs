use std::cmp;
use std::cmp::Ordering;

use crossterm::event::KeyCode;

use crate::editor_content_display::Row;
use crate::{EditorContentDisplay, TAB_STOP};

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
    // 渲染列偏移量
    render_x: usize,
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
            render_x: 0,
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
    pub fn scroll(&mut self, ecd: &EditorContentDisplay) {
        self.render_x = 0;
        if self.cursor_y < ecd.number_of_rows() {
            self.render_x = self.calculate_render_x(ecd.get_row(self.cursor_y));
        }
        // 行偏移量变化
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
        // 列偏移量变化
        self.column_offset = cmp::min(self.column_offset, self.render_x);
        if self.render_x >= self.column_offset + self.screen_columns {
            self.column_offset = self.render_x - self.screen_columns + 1;
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
                if self.cursor_x != 0 {
                    self.cursor_x -= 1;
                } else {
                    self.cursor_y -= 1;
                    self.cursor_x = ecd.get_render_row(self.cursor_y).len();
                }
            }
            KeyCode::Right => {
                if self.cursor_y < number_of_rows {
                    match self.cursor_x.cmp(&ecd.get_render_row(self.cursor_y).len()) {
                        Ordering::Less => self.cursor_x += 1,
                        _ => {
                            self.cursor_y += 1;
                            self.cursor_x = 0;
                        }
                    }
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
                if self.cursor_y < number_of_rows {
                    self.cursor_x = ecd.get_render_row(self.cursor_y).len();
                }
                self.cursor_x = self.screen_columns - 1;
            }
            _ => unimplemented!(),
        }
        let row_len = if self.cursor_y < number_of_rows {
            ecd.get_render_row(self.cursor_y).len()
        } else {
            0
        };
        self.cursor_x = cmp::min(self.cursor_x, row_len);
    }

    pub fn get_render_x(&self) -> usize {
        self.render_x
    }

    fn calculate_render_x(&self, row: &Row) -> usize {
        row.get_row_content()[..self.cursor_x]
            .chars()
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + ((TAB_STOP - 1) - (render_x % TAB_STOP) + 1)
                } else {
                    render_x + 1
                }
            })
    }
}
