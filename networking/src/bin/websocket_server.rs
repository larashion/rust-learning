use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;

const SERVER_ADDR: &str = "127.0.0.1:9001";

#[tokio::main]
async fn main() {
    println!("=== WebSocket Server ===");
    run_server().await;
}

// ============================================================================ 
// 部分 1: WebSocket 服务端 (Echo Server)
// ============================================================================ 
async fn run_server() {
    let listener = TcpListener::bind(SERVER_ADDR).await.expect("Failed to bind");
    println!("Server: 监听于 ws://{}", SERVER_ADDR);

    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(async move {
            println!("Server: 接受连接来自 {}", addr);
            handle_connection(stream).await;
            println!("Server: 连接断开 {}", addr);
        });
    }
}

async fn handle_connection(raw_stream: TcpStream) {
    // 将 TCP 流升级为 WebSocket 流
    let ws_stream = accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake");

    // 将流拆分为 发送端(write) 和 接收端(read) 
    let (mut write, mut read) = ws_stream.split();

    // 循环处理接收到的每一条消息
    while let Some(msg_result) = read.next().await {
        match msg_result {
            Ok(msg) => {
                if msg.is_text() || msg.is_binary() {
                    println!("Server: 收到消息: {}", msg);
                    // Echo: 原样发回给客户端
                    if let Err(e) = write.send(msg).await {
                        eprintln!("Server: 发送失败: {}", e);
                        break;
                    }
                } else if msg.is_close() {
                    break;
                }
            }
            Err(e) => {
                eprintln!("Server: 连接错误: {}", e);
                break;
            }
        }
    }
}
