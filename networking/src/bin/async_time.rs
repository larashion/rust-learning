use tokio::time::{sleep, Duration, timeout};

#[tokio::main]
async fn main() {
    example4_timer().await;
}

async fn example4_timer() {
    println!("开始计时");
    sleep(Duration::from_secs(1)).await;
    println!("1 秒后");
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for i in 0..3 {
        interval.tick().await;
        println!("定时器 tick {}", i + 1);
    }
    match timeout(Duration::from_secs(2), sleep(Duration::from_secs(3))).await {
        Ok(_) => println!("任务完成"),
        Err(_) => println!("任务超时"),
    }
}