use std::cmp;
use std::cmp::Ordering;

use crossterm::event::KeyCode;

use crate::view::Row;
use crate::{EditorView, TAB_SIZE};

/// 光标控制器
pub struct CursorController {
    // 窗口最大行数
    screen_rows: usize,
    // 窗口最大列数
    screen_columns: usize,
    // 行偏移量
    rows_offset: usize,
    // 列偏移量
    columns_offset: usize,
    // 原内容光标位置
    raw_position: Cursor,
    // 渲染内容光标位置
    render_position: Cursor,
}

impl CursorController {
    /// 创建初始光标控制器
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            screen_rows: win_size.1,
            screen_columns: win_size.0,
            rows_offset: 0,
            columns_offset: 0,
            raw_position: Cursor(0, 0),
            render_position: Cursor(0, 0),
        }
    }

    /// 获取当前光标位置元组，索引 0 为列，索引 1 为行
    pub fn get_cursor(&mut self) -> &mut Cursor {
        &mut self.render_position
    }

    /// 获取当前行偏移量
    pub fn get_rows_offset(&self) -> usize {
        self.rows_offset
    }

    pub fn get_columns_offset(&self) -> usize {
        self.columns_offset
    }

    /// 屏幕滚动
    pub fn scroll(&mut self, ecd: &EditorView) {
        // 设置渲染列偏移量
        self.render_position.0 = if self.render_position.1 < ecd.number_of_rows() {
            self.calculate_render_x(ecd.get_row(self.render_position.1))
        } else {
            0
        };

        // 设置行偏移量
        self.rows_offset = if self.render_position.1 >= self.rows_offset + self.screen_rows {
            self.render_position.1 - self.screen_rows + 1
        } else {
            cmp::min(self.rows_offset, self.render_position.1)
        };

        // 设置列偏移量
        self.columns_offset = if self.render_position.0 >= self.columns_offset + self.screen_columns
        {
            self.render_position.0 - self.screen_columns + 1
        } else {
            cmp::min(self.columns_offset, self.render_position.0)
        }
    }

    /// 移动光标
    pub fn move_cursor(&mut self, direction: KeyCode, ecd: &EditorView) {
        let number_of_rows = ecd.number_of_rows();
        // todo 移动光标支持中文
        // todo 支持原始光标移动
        match direction {
            KeyCode::Up => {
                self.render_position.1 = self.render_position.1.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.render_position.0 != 0 {
                    self.render_position.0 -= 1;
                } else {
                    self.render_position.1 -= 1;
                    self.render_position.0 = ecd.get_render_row(self.render_position.1).len();
                }
            }
            KeyCode::Right => {
                if self.render_position.1 < number_of_rows {
                    match self
                        .render_position
                        .0
                        .cmp(&ecd.get_render_row(self.render_position.1).len())
                    {
                        Ordering::Less => self.render_position.0 += 1,
                        _ => {
                            self.render_position.1 += 1;
                            self.render_position.0 = 0;
                        }
                    }
                }
            }
            KeyCode::Down => {
                if self.render_position.1 < number_of_rows {
                    self.render_position.1 += 1;
                }
            }
            KeyCode::Home => {
                self.render_position.0 = 0;
            }
            KeyCode::End => {
                self.render_position.0 = ecd.get_render_row(self.render_position.1).len();
            }
            _ => unimplemented!(),
        }

        // 光标上下移动时，重新设置列偏移量
        let row_len = if self.render_position.1 < number_of_rows {
            ecd.get_render_row(self.render_position.1).len()
        } else {
            0
        };
        self.render_position.0 = cmp::min(self.render_position.0, row_len);
    }

    fn calculate_render_x(&self, row: &Row) -> usize {
        row.get_row_content()[..self.raw_position.0]
            .chars()
            .fold(0, |render_x, c| {
                if c == '\t' {
                    render_x + ((TAB_SIZE - 1) - (render_x % TAB_SIZE) + 1)
                } else {
                    render_x + 1
                }
            })
    }
}

/// 光标
pub struct Cursor(usize, usize);

impl Cursor {
    /// 获取光标 x 轴偏移量
    pub fn get_x(&self) -> usize {
        self.0
    }

    pub fn set_x(&mut self, new_x: usize) {
        self.0 = new_x;
    }

    /// 获取光标 y 轴偏移量
    pub fn get_y(&self) -> usize {
        self.1
    }

    pub fn set_y(&mut self, new_y: usize) {
        self.1 = new_y;
    }
}
