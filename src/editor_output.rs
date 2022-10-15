use std::io;
use std::io::{stdout, Write};

/// 编辑器输出
pub struct EditorOutput {
    // 输出内容缓冲区
    content: String,
}

impl EditorOutput {
    /// 创建一个编辑器内容输出
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    /// 添加一个字符到编辑器内容输出缓冲区
    pub fn push(&mut self, ch: char) {
        self.content.push(ch)
    }

    /// 添加一个字符串到编辑器内容输出缓冲区
    pub fn push_str(&mut self, string: &str) {
        self.content.push_str(string)
    }
}

impl Write for EditorOutput {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match std::str::from_utf8(buf) {
            Ok(s) => {
                self.content.push_str(s);
                Ok(s.len())
            }
            Err(_) => Err(io::ErrorKind::WriteZero.into()),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let out = write!(stdout(), "{}", self.content);
        stdout().flush()?;
        self.content.clear();
        out
    }
}
