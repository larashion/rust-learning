use tokio::sync::broadcast;
use tokio::sync::broadcast::error::RecvError;

#[tokio::main]
async fn main() {
    println!("=== Broadcast Channel 示例 ===");
    broadcast().await;
}
async fn broadcast() {
    // 一对多广播
    // 配置更新、聊天室消息、事件总线。
    let (tx, rx1) = broadcast::channel(10);
    let rx2 = tx.subscribe();

    let _ = tokio::join!(supplier(tx), consumer(1, rx1), consumer(2, rx2));
}
async fn consumer(id: usize, mut rx: broadcast::Receiver<usize>) {
    loop {
        // 异步阻塞接收消息
        match rx.recv().await {
            Ok(msg) => println!("消费者 {} 收到消息 {}", id, msg),

            Err(RecvError::Lagged(skipped)) => {
                println!("消费者 {} 滞后，跳过了 {} 条消息", id, skipped);
                // 接着处理当前最新的数据
                continue;
            }
            Err(RecvError::Closed) => {
                // 通道关闭，每一个正在运行的消费者任务都必须独立地收到通知并退出循环，
                // 否则程序就会出现资源泄漏或死循环。
                println!("消费者 {} 通道关闭 ", id);
                break;
            }
        }
    }
}
async fn supplier(tx: broadcast::Sender<usize>) {
    for i in 1..9 {
        // 无阻塞地发送消息
        if tx.send(i).is_err() {
            println!("接收端全部断开");
            break;
        }
        println!("广播发送: {}", i);
    }
}
