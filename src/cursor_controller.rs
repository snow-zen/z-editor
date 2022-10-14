use crossterm::event::KeyCode;
use std::cmp;

/// 光标控制器
pub struct CursorController {
    cursor_x: usize,
    cursor_y: usize,
    screen_rows: usize,
    screen_columns: usize,
    row_offset: usize,
}

impl CursorController {
    pub fn new(win_size: (usize, usize)) -> Self {
        Self {
            cursor_x: 0,
            cursor_y: 0,
            screen_rows: win_size.1,
            screen_columns: win_size.0,
            row_offset: 0,
        }
    }

    pub fn get_cursor(&self) -> (usize, usize) {
        (self.cursor_x, self.cursor_y)
    }

    pub fn get_row_offset(&self) -> usize {
        self.row_offset
    }

    pub fn scroll(&mut self) {
        self.row_offset = cmp::min(self.row_offset, self.cursor_y);
        if self.cursor_y >= self.row_offset + self.screen_rows {
            self.row_offset = self.cursor_y - self.screen_rows + 1;
        }
    }

    pub fn move_cursor(&mut self, direction: KeyCode, number_of_rows: usize) {
        match direction {
            KeyCode::Up => {
                self.cursor_y = self.cursor_y.saturating_sub(1);
            }
            KeyCode::Left => {
                self.cursor_x = self.cursor_x.saturating_sub(1);
            }
            KeyCode::Right => {
                if self.cursor_x != self.screen_columns - 1 {
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
    }
}
