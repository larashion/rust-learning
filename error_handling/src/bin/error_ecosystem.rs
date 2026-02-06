// ============================================================================
// Error Handling - 生态系统 (thiserror & anyhow)
// ============================================================================
//
// 现代 Rust 错误处理：
// 1. thiserror: 用于库 (Library)开发。
//    - 为枚举自动实现 std::error::Error trait。
//    - 不会带来运行时开销。
// 2. anyhow: 用于应用 (Application)开发。
//    - 方便地包装任何错误 (Box<dyn Error>)。
//    - 提供 context() 方法添加上下文信息。
//    - 漂亮的错误打印。

use anyhow::{Context, Result};

// ============================================================================
// 部分 1:  使用 thiserror
// ============================================================================
mod my_library {
    use std::io;
    use std::path::Path;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum DataStoreError {
        #[error("数据读取失败")]
        Io(#[from] io::Error),

        #[error("数据格式错误: {0}")]
        Format(String),

        #[allow(dead_code)]
        #[error("未找到键值: {0}")]
        NotFound(String),

        // 通常建议将 unknown 作为兜底
        #[allow(dead_code)]
        #[error("未知错误")]
        Unknown,
    }

    // 2. 使用 impl AsRef<Path> 让函数更通用
    pub fn read_data<P: AsRef<Path>>(path: P) -> Result<String, DataStoreError> {
        // path.as_ref() 转换为 &Path
        let content = std::fs::read_to_string(path.as_ref())?;

        if content.is_empty() {
            return Err(DataStoreError::Format("文件为空".into()));
        }

        Ok(content)
    }
}

// ============================================================================
// 部分 2: 使用 anyhow
// ============================================================================
// 注意：anyhow::Result<T> 等价于 Result<T, anyhow::Error>

fn run_application() -> Result<()> {
    println!("--- 开始运行应用 ---");

    let filename = "non_existent_config.toml";

    // 使用 .context() 为错误添加更多语义信息
    // 这样当报错时，我们可以看到：
    // "加载配置文件失败" -> "数据读取失败" -> "No such file or directory"
    // 就像剥洋葱一样清晰
    let content =
        my_library::read_data(filename).context(format!("加载配置文件 '{}' 失败", filename))?;

    println!("配置内容: {}", content);
    Ok(())
}

fn main() {
    // 在 main 中捕获 anyhow 错误 打印错误链（Cause Chain）
    if let Err(e) = run_application() {
        println!("\n❌ 应用发生错误:\n{:?}", e);
    }
}
