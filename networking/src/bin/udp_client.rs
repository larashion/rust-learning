use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    println!("=== UDP Client Example ===");
    example5_udp_client()
}

// ============================================================================ 
// 示例 5: UDP 客户端
// ============================================================================ 
fn example5_udp_client() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;

    // 发送消息
    let message = "你好，UDP 服务端！";
    let server_addr = "127.0.0.1:8082";
    socket.send_to(message.as_bytes(), server_addr)?;
    println!("UDP 发送: {}", message);

    // 接收响应
    let mut buffer = [0; 1024];
    let (bytes_read, _) = socket.recv_from(&mut buffer)?;
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("收到响应: {}", response);

    Ok(())
}
