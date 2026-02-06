use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() -> io::Result<()> {
    println!("=== TCP Server Example ===");
    example1_tcp_server()
}

// ============================================================================ 
// 示例 1: TCP 服务端基本实现
// ============================================================================ 
fn example1_tcp_server() -> io::Result<()> {
    // 绑定到本地端口 8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("TCP 服务端监听在: 127.0.0.1:8080");

    // 接受连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("新连接: {:?}", stream.peer_addr()?);
                
                // 使用线程处理每个连接，避免阻塞主线程
                thread::spawn(|| {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("客户端处理错误: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    // 读取数据
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("收到消息: {}", message);

    // 发送响应
    let response = "你好，客户端！";
    stream.write_all(response.as_bytes())?;

    Ok(())
}
