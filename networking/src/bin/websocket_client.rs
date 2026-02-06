use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const SERVER_ADDR: &str = "127.0.0.1:9001";

#[tokio::main]
async fn main() {
    println!("=== WebSocket Client ===");
    run_client().await;
}

// ============================================================================ 
// 部分 2: WebSocket 客户端
// ============================================================================ 
async fn run_client() {
    let url = format!("ws://{}", SERVER_ADDR);
    println!("Client: 正在连接到 {}", url);

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("Client: 连接成功！");

    let (mut write, mut read) = ws_stream.split();

    // 1. 启动一个任务用于接收消息 (并在后台打印)
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => println!("Client: 收到回显 -> {}", msg),
                Err(e) => eprintln!("Client: 接收错误: {}", e),
            }
        }
    });

    // 2. 发送几条测试消息
    let messages = vec![
        "Hello, WebSocket!",
        "Rust is awesome!",
        "Bye bye!",
    ];

    for msg in messages {
        println!("Client: 发送 -> \"{}\"", msg);
        write.send(Message::Text(msg.to_string())).await.unwrap();
        sleep(Duration::from_millis(500)).await;
    }

    // 3. 发送关闭帧
    println!("Client: 发送关闭请求");
    write.close().await.unwrap();

    // 等待接收任务结束 (Server 关闭连接后 read 会返回 None)
    let _ = recv_task.await;
    println!("Client: 任务结束");
}
