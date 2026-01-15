// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================ 
// Error Handling - 生态系统 (thiserror & anyhow)
// ============================================================================ 
//
// 现代 Rust 错误处理黄金搭档：
// 1. thiserror: 用于【库 (Library)】开发。
//    - 为枚举自动实现 std::error::Error trait。
//    - 不会带来运行时开销。
// 2. anyhow: 用于【应用 (Application)】开发。
//    - 方便地包装任何错误 (Box<dyn Error>)。
//    - 提供 context() 方法添加上下文信息。
//    - 漂亮的错误打印。

use anyhow::{Context, Result};

// ============================================================================ 
// 部分 1: 模拟库代码 (使用 thiserror)
// ============================================================================ 
mod my_library {
    use thiserror::Error;
    use std::io;

    // 自定义错误枚举
    #[derive(Error, Debug)]
    pub enum DataStoreError {
        // #[error("...")] 定义了 Display 的输出
        #[error("数据读取失败")]
        IoError(#[from] io::Error), // #[from] 自动生成 From<io::Error> 实现
        
        #[error("数据格式错误: {0}")]
        FormatError(String),
        
        #[allow(dead_code)]
        #[error("未找到键值: {0}")]
        NotFound(String),
        
        #[allow(dead_code)]
        #[error("未知错误")]
        Unknown,
    }

    pub fn read_data(path: &str) -> Result<String, DataStoreError> {
        // 这里的 io::Error 会自动转换为 DataStoreError::IoError
        let content = std::fs::read_to_string(path)?;
        
        if content.is_empty() {
            return Err(DataStoreError::FormatError("文件为空".into()));
        }
        
        Ok(content)
    }
}

// ============================================================================ 
// 部分 2: 模拟应用代码 (使用 anyhow)
// ============================================================================ 
// 注意：anyhow::Result<T> 等价于 Result<T, anyhow::Error>

fn run_application() -> Result<()> {
    println!("--- 开始运行应用 ---");

    let filename = "non_existent_config.toml";

    // 使用 .context() 为错误添加更多语义信息
    // 这样当报错时，我们可以看到：
    // "加载配置文件失败" -> "数据读取失败" -> "No such file or directory"
    // 就像剥洋葱一样清晰
    let content = my_library::read_data(filename)
        .context(format!("加载配置文件 '{}' 失败", filename))?;

    println!("配置内容: {}", content);
    Ok(())
}

fn main() {
    // 在 main 中捕获 anyhow 错误
    if let Err(e) = run_application() {
        println!("\n❌ 应用发生错误:\n{:?}", e);
        
        // 演示：如果使用了 anyhow，{:?} 会打印出完整的错误链（Cause Chain）
        // 输出示例：
        // ❌ 应用发生错误:
        // 加载配置文件 'non_existent_config.toml' 失败
        // 
        // Caused by:
        //     0: 数据读取失败
        //     1: The system cannot find the file specified. (os error 2)
    }
}