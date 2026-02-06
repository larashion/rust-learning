use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== 异步 Semaphore 示例 ===");
    let semaphore = Arc::new(Semaphore::new(3)); // 允许 3 个并发
    let mut handles = vec![];

    for i in 0..10 {
        let semaphore = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("任务 {} 获得许可，开始执行", i);
            sleep(Duration::from_millis(500)).await;
            println!("任务 {} 完成，释放许可", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
}
