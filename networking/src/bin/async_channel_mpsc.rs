use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    println!("=== MPSC Channel 示例 ===");
    // 任务队列、Actor 模型、日志收集。
    mpsc_demo().await;
}

async fn mpsc_demo() {
    let (tx1, rx) = mpsc::channel::<usize>(100);
    let tx2 = tx1.clone();
    // 后台任务处理
    // 这里不能 .await
    let handle1 = tokio::spawn(supplier(tx1));
    let handle2 = tokio::spawn(supplier(tx2));
    // 前台日志、结果收集、监听
    // 直接 .await
    // 此时3个任务并行执行
    consumer(rx).await;
    // 收尾
    let _ = handle1.await;
    let _ = handle2.await;
    println!("所有任务完成");
}

async fn supplier(tx: mpsc::Sender<usize>) {
    for i in 1..4 {
        if tx.send(i).await.is_err() {
            eprintln!("发送错误: 接收端已关闭");
            break;
        }
        println!("发送: {}", i);
    }
}

async fn consumer(mut rx: mpsc::Receiver<usize>) {
    // 消费者阻塞接收，直到通道关闭
    while let Some(msg) = rx.recv().await {
        println!("接收: {}", msg);
    }
    println!("通道耗尽，已关闭");
}
