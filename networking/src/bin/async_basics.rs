use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    example_spawn().await;
}

async fn example_spawn() {
    println!("主任务");
    // tokio::spawn 立即将任务提交到 tokio 运行时的任务队列
    // 任务会尽快被调度执行，但不会阻塞当前执行流
    // tokio 默认使用工作窃取调度器，主动调度到可用的工作线程上
    let task1 = tokio::spawn(async {
        println!("任务 1 开始");
        sleep(Duration::from_secs(1)).await;
        println!("任务 1 完成");
    });
    let task2 = tokio::spawn(async {
        println!("任务 2 开始");
        sleep(Duration::from_secs(2)).await;
        println!("任务 2 完成");
    });
    println!("主任务继续");
    // await task1：当前异步任务挂起，等待 task1 完成
    // 这会让出当前线程的执行权，线程可以执行其他任务（非阻塞）
    task1.await.unwrap();
    task2.await.unwrap();
    println!("所有任务完成");
}
