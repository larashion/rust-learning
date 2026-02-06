use std::io;
use std::net::UdpSocket;

fn main() -> io::Result<()> {
    println!("=== UDP Server Example ===");
    example4_udp_server()
}

// ============================================================================ 
// 示例 4: UDP 服务端
// ============================================================================ 
fn example4_udp_server() -> io::Result<()> {
    // 绑定 UDP Socket
    let socket = UdpSocket::bind("127.0.0.1:8082")?;
    println!("UDP 服务端监听在: 127.0.0.1:8082");

    let mut buffer = [0; 1024];

    loop {
        // 接收数据
        let (bytes_read, src_addr) = socket.recv_from(&mut buffer)?;
        let message = String::from_utf8_lossy(&buffer[..bytes_read]);
        println!("从 {} 收到: {}", src_addr, message);

        // 发送响应
        let response = "UDP 响应";
        socket.send_to(response.as_bytes(), src_addr)?;
    }
}
