use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::time::{sleep, timeout, Duration};

#[tokio::main]
async fn main() {
    println!("=== 异步重试机制示例 ===");
    
    // 演示基本超时重试
    println!("\n--- 简单超时 ---");
    match timeout(Duration::from_secs(5), unreliable_operation()).await {
        Ok(Ok(result)) => println!("成功: {}", result),
        Ok(Err(e)) => println!("失败: {}", e),
        Err(_) => println!("超时"),
    }

    // 演示带指数退避的重试
    println!("\n--- 指数退避重试 ---");
    
    // 使用 Arc<Mutex> 在多次闭包调用间共享状态
    let attempt_counter = Arc::new(Mutex::new(0));
    
    // 我们需要构造一个闭包，每次调用返回一个新的 Future
    let operation = || -> Pin<Box<dyn Future<Output = Result<i32, &'static str>> + Send>> {
        let counter = Arc::clone(&attempt_counter);
        Box::pin(async move {
            let mut num = counter.lock().unwrap();
            *num += 1;
            
            if *num <= 2 {
                println!("操作失败 (尝试次数: {})", *num);
                Err("临时错误")
            } else {
                println!("操作成功！(尝试次数: {})", *num);
                Ok(100)
            }
        })
    };

    match retry_with_backoff(operation, 3, Duration::from_secs(1)).await {
        Ok(val) => println!("最终结果: {}", val),
        Err(e) => println!("最终失败: {}", e),
    }
}

async fn unreliable_operation() -> Result<i32, &'static str> {
    sleep(Duration::from_millis(100)).await;
    Ok(42)
}

async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
    initial_delay: Duration,
) -> Result<T, E>
where
    E: std::fmt::Debug,
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
{
    let mut delay = initial_delay;
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries => {
                println!("重试逻辑捕获错误: {:?}，{} 秒后重试", e, delay.as_secs());
                sleep(delay).await;
                delay *= 2;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}