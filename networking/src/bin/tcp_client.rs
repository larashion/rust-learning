use std::io::{self, Read, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    println!("=== TCP Client Example ===");
    example2_tcp_client()
}

// ============================================================================ 
// 示例 2: TCP 客户端基本实现
// ============================================================================ 
fn example2_tcp_client() -> io::Result<()> {
    // 连接到服务端
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("已连接到服务端");

    // 发送消息
    let message = "你好，服务端！";
    stream.write_all(message.as_bytes())?;
    println!("已发送: {}", message);

    // 读取响应
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("收到响应: {}", response);

    Ok(())
}
