use std::path::PathBuf;
use crate::status::Status::Saved;

/// 状态信息
pub struct StatusInfo {
    // 文件名
    file_name: Option<PathBuf>,
    // 当前行数
    lines: usize,
    // 状态
    status: Status,
    // 显示信息
    message: String
}

impl StatusInfo {
    /// 创建状态信息
    pub fn new(file_name: Option<PathBuf>, lines: usize, initial_message: String) -> Self {
        Self {
            file_name,
            lines,
            status: Saved,
            message: initial_message
        }
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

