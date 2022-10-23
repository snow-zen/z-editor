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
    // todo 考虑将 EditView 的只读借用加入结构体中
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
        self.rows_offset = if self.render_position.1 >= self.rows_offset + ecd.get_win_max_rows() {
            self.render_position.1 - ecd.get_win_max_rows() + 1
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
        debug!("光标变动前位置：{}", self.render_position);
        match direction {
            KeyCode::Up => {
                self.move_up();
            }
            KeyCode::Left => {
                self.move_left(ecd);
            }
            KeyCode::Right => {
                self.move_right(ecd);
            }
            KeyCode::Down => {
                self.move_down(ecd);
            }
            KeyCode::Home => {
                self.move_home();
            }
            KeyCode::End => {
                self.move_end(ecd);
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

    /// 光标上移
    pub fn move_up(&mut self) {
        self.render_position.1 = self.render_position.1.saturating_sub(1);
        self.raw_position.1 = self.raw_position.1.saturating_sub(1);
    }

    /// 光标下移
    pub fn move_down(&mut self, ecd: &EditorView) {
        if self.render_position.1 < ecd.number_of_rows() {
            self.render_position.1 += 1;
            self.raw_position.1 += 1;
        }
    }

    /// 光标左移
    pub fn move_left(&mut self, ecd: &EditorView) {
        if self.render_position.0 > 0 {
            self.render_position.0 -= 1;
            let raw_content = ecd.raw_content_of_row(self.raw_position.1);
            self.raw_position.0 = (0..self.raw_position.0)
                .rposition(|i| raw_content.is_char_boundary(i))
                .unwrap();
        } else {
            // 文首左移切换到上一行文末
            // todo 考虑边界情况，例如首行文首左移
            self.render_position.1 -= 1;
            self.raw_position.1 -= 1;
            self.render_position.0 = ecd.rendered_content_of_row(self.render_position.1).len();
            self.raw_position.0 = ecd.raw_content_of_row(self.raw_position.1).len();
        }
    }

    /// 光标右移
    pub fn move_right(&mut self, ecd: &EditorView) {
        if self.render_position.1 < ecd.number_of_rows() {
            match self
                .render_position
                .0
                .cmp(&ecd.rendered_content_of_row(self.render_position.1).len())
            {
                Ordering::Less => {
                    self.render_position.0 += 1;
                    let raw_text = ecd.raw_content_of_row(self.raw_position.1);
                    self.raw_position.0 += ((self.raw_position.0 + 1)..(self.raw_position.0 + 3))
                        .position(|i| raw_text.is_char_boundary(i))
                        .unwrap()
                        + 1;
                }
                _ => {
                    self.render_position.1 += 1;
                    self.raw_position.1 += 1;
                    self.render_position.0 = 0;
                    self.raw_position.0 = 0;
                }
            }
        }
    }

    /// 光标移动至行首
    pub fn move_home(&mut self) {
        self.render_position.0 = 0;
        self.raw_position.0 = 0;
    }

    /// 光标移动至行末
    pub fn move_end(&mut self, ecd: &EditorView) {
        self.render_position.0 = ecd.rendered_content_of_row(self.render_position.1).len();
        self.raw_position.0 = ecd.raw_content_of_row(self.raw_position.1).len();
    }

    /// 光标上翻页
    pub fn move_page_up(&mut self, ecd: &EditorView) {
        self.render_position.1 = self.rows_offset;
        (0..ecd.get_win_max_rows()).for_each(|_| {
            self.move_up();
        })
    }

    /// 光标下翻页
    pub fn move_page_down(&mut self, ecd: &EditorView) {
        self.render_position.1 = cmp::min(
            self.rows_offset + ecd.get_win_max_rows() + 1,
            ecd.number_of_rows(),
        );
        (0..ecd.get_win_max_rows()).for_each(|_| {
            self.move_down(ecd);
        })
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
