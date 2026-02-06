use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    println!("=== 异步 RwLock 示例 ===");
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 启动多个读者
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let r = data.read().await;
            println!("读者 {}: {}", i, *r);
        });
        handles.push(handle);
    }

    // 启动写者
    let writer_data = Arc::clone(&data);
    let handle = tokio::spawn(async move {
        let mut w = writer_data.write().await;
        *w = 100;
        println!("写者: 更新数据为 {}", *w);
    });
    handles.push(handle);

    for handle in handles {
        handle.await.unwrap();
    }
    println!("最终值: {}", *data.read().await);
}
