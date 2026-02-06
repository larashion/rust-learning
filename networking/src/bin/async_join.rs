use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("=== Tokio Join 示例 ===");
    let (result1, result2) = tokio::join!(task1(), task2());
    println!("结果: {} {}", result1, result2);
}

async fn task1() -> i32 {
    sleep(Duration::from_secs(1)).await;
    1
}

async fn task2() -> i32 {
    sleep(Duration::from_secs(2)).await;
    2
}
