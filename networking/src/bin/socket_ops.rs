use std::io;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener};

fn main() -> io::Result<()> {
    println!("=== Socket Operations Examples ===");

    println!("
--- 示例 6: SocketAddr 操作 ---");
    example6_socket_address();

    println!("
--- 示例 10: 检查端口是否可用 ---");
    example10_check_port_available(8080).unwrap();
    example10_check_port_available(9999).unwrap();

    println!("
--- 示例 11: 获取本地 IP 地址 ---");
    example11_local_ip();

    Ok(())
}

// ============================================================================ 
// 示例 6: SocketAddr 操作
// ============================================================================ 
fn example6_socket_address() {
    // IPv4 地址
    let ipv4 = Ipv4Addr::new(127, 0, 0, 1);
    println!("IPv4 地址: {}", ipv4);

    // SocketAddrV4
    let socket_v4 = SocketAddrV4::new(ipv4, 8080);
    println!("SocketAddrV4: {}", socket_v4);

    // 从字符串解析
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    println!("解析的 SocketAddr: {}", addr);
    println!("  IP: {}", addr.ip());
    println!("  端口: {}", addr.port());

    // 判断 IP 类型
    match addr {
        SocketAddr::V4(_v4) => println!("  是 IPv4"),
        SocketAddr::V6(_v6) => println!("  是 IPv6"),
    }
}

// ============================================================================ 
// 示例 10: 检查端口是否可用
// ============================================================================ 
fn example10_check_port_available(port: u16) -> io::Result<bool> {
    let addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    match TcpListener::bind(addr) {
        Ok(_) => {
            println!("端口 {} 可用", port);
            Ok(true)
        }
        Err(_) => {
            println!("端口 {} 已被占用", port);
            Ok(false)
        }
    }
}

// ============================================================================ 
// 示例 11: 获取本地 IP 地址
// ============================================================================ 
fn example11_local_ip() {
    match TcpListener::bind("0.0.0.0:0") {
        Ok(listener) => {
            let local_addr = listener.local_addr().unwrap();
            println!("本地 IP 地址: {}", local_addr.ip());
        }
        Err(e) => {
            eprintln!("获取本地 IP 失败: {}", e);
        }
    }
}
