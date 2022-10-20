use crate::status::Status::Saved;
use std::path::PathBuf;

/// 状态信息
pub struct StatusInfo {
    // 文件名
    file_name: Option<PathBuf>,
    // 当前行数
    lines: usize,
    // 状态
    status: Status,
    // 显示信息
    message: String,
}

impl StatusInfo {
    /// 创建状态信息
    pub fn new(file_name: Option<PathBuf>, lines: usize, initial_message: String) -> Self {
        Self {
            file_name,
            lines,
            status: Saved,
            message: initial_message,
        }
    }

    /// 获取文件名。如果不存在，则返回默认名称：**[No Name]**
    pub fn file_name_or_default(&self) -> &str {
        self.file_name
            .as_ref()
            .and_then(|path| path.file_name())
            .and_then(|name| name.to_str())
            .unwrap_or("[No Name]")
    }

    pub fn get_message(&self) -> &str {
        self.message.as_str()
    }
}

/// 状态枚举
pub enum Status {
    /// 已保存
    Saved,
    /// 只读
    ReadOnly,
    /// 已修改
    Modified,
}
