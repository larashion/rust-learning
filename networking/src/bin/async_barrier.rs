use std::sync::Arc;
use tokio::sync::Barrier;

#[tokio::main]
async fn main() {
    // 模拟结构化并发（简易版）
    println!("=== 异步 Barrier 示例 ===");
    // 3个任务到达后才会放行
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];

    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            // 实际场景可能有任务逃逸
            println!("任务 {} 准备中...", i);
            // .await 挂起点
            barrier.wait().await;
            println!("任务 {} 继续执行", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        // .await 等待 Future 结果
        handle.await.unwrap();
    }
}
