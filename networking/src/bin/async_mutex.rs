use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    println!("=== 异步 Mutex 示例 ===");
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut num = data.lock().await;
            *num += 1;
            println!("计数: {}", *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }
    println!("最终计数: {}", *data.lock().await);
}
