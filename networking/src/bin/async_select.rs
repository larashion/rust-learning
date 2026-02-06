use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Tokio Select 示例 ===");
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);

    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        let _ = tx1.send("来自通道 1").await;
    });

    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        let _ = tx2.send("来自通道 2").await;
    });

    // 演示 select! 只要有一个分支完成就会返回
    tokio::select! {
        Some(msg) = rx1.recv() => {
            println!("通道 1 收到: {:?}", msg);
        }
        Some(msg) = rx2.recv() => {
            println!("通道 2 收到: {:?}", msg);
        }
        else => {
            println!("所有通道已关闭");
        }
    }
}
