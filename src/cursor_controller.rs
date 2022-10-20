use std::cmp;
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use crossterm::event::KeyCode;

use crate::view::EditRow;
use crate::{EditorView, TAB_SIZE};

/// 光标控制器
pub struct CursorController {
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
    pub fn new() -> Self {
        Self {
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
        let win_size = ecd.get_win_size();
        // 设置渲染列偏移量
        self.render_position.0 = if self.render_position.1 < ecd.number_of_rows() {
            self.calculate_render_x(ecd.get_edit_row(self.render_position.1))
        } else {
            0
        };

        // 设置行偏移量
        self.rows_offset = if self.render_position.1 >= self.rows_offset + win_size.1 {
            self.render_position.1 - win_size.1 + 1
        } else {
            cmp::min(self.rows_offset, self.render_position.1)
        };

        // 设置列偏移量
        self.columns_offset = if self.render_position.0 >= self.columns_offset + win_size.0 {
            self.render_position.0 - win_size.0 + 1
        } else {
            cmp::min(self.columns_offset, self.render_position.0)
        };
        trace!(
            "屏幕发生滚动，渲染列偏移量：{}，行偏移量：{}，列偏移量：{}",
            self.render_position.0,
            self.rows_offset,
            self.columns_offset
        );
    }

    /// 移动光标
    pub fn move_cursor(&mut self, direction: KeyCode, ecd: &EditorView) {
        let number_of_rows = ecd.number_of_rows();
        // todo 移动光标支持中文
        // todo 支持原始光标移动
        debug!("光标变动前位置：{}", self.render_position);
        match direction {
            KeyCode::Up => {
                self.render_position.1 = self.render_position.1.saturating_sub(1);
            }
            KeyCode::Left => {
                if self.render_position.0 != 0 {
                    self.render_position.0 -= 1;
                } else {
                    self.render_position.1 -= 1;
                    self.render_position.0 =
                        ecd.rendered_content_of_row(self.render_position.1).len();
                }
            }
            KeyCode::Right => {
                if self.render_position.1 < number_of_rows {
                    match self
                        .render_position
                        .0
                        .cmp(&ecd.rendered_content_of_row(self.render_position.1).len())
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
                self.render_position.0 = ecd.rendered_content_of_row(self.render_position.1).len();
            }
            _ => unimplemented!(),
        }

        // 光标上下移动时，重新设置列偏移量
        let row_len = if self.render_position.1 < number_of_rows {
            ecd.rendered_content_of_row(self.render_position.1).len()
        } else {
            0
        };
        self.render_position.0 = cmp::min(self.render_position.0, row_len);
        debug!("光标变动后位置：{}", self.render_position);
    }

    fn calculate_render_x(&self, row: &EditRow) -> usize {
        row.get_raw_content()[..self.raw_position.0]
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
#[derive(Debug)]
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

impl Display for Cursor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
