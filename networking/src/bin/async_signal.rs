#[tokio::main]
async fn main() {
    println!("=== 异步信号处理示例 ===");
    handle_signal().await;
}

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

#[cfg(unix)]
async fn handle_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    println!("等待 SIGTERM 信号 (Unix)...");
    println!("提示: 你可以在另一个终端运行 'kill -TERM <pid>' 来触发");
    
    // 同时也监听 Ctrl+C 作为演示
    tokio::select! {
        _ = sigterm.recv() => println!("收到 SIGTERM，优雅关闭"),
        _ = tokio::signal::ctrl_c() => println!("收到 Ctrl+C，优雅关闭"),
    }
}

#[cfg(windows)]
async fn handle_signal() {
    println!("Windows 平台: 监听 Ctrl+C (SIGINT)");
    println!("请按下 Ctrl+C...");
    match tokio::signal::ctrl_c().await {
        Ok(()) => println!("收到 Ctrl+C，优雅关闭"),
        Err(err) => eprintln!("监听信号失败: {}", err),
    }
}
