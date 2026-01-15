// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================ 
// TCP/UDP 网络编程基础 (std::net)
// ============================================================================ 
//
// Rust 标准库提供了基础的网络编程功能。
//
// 主要特点：
// 1. std::net - 同步网络 I/O
// 2. TcpListener - TCP 服务端监听器
// 3. TcpStream - TCP 连接流
// 4. UdpSocket - UDP Socket
// 5. IpAddr, SocketAddr - 网络地址类型

use std::io::{self, Read, Write, BufRead, BufReader};
use std::net::{TcpListener, TcpStream, UdpSocket, Ipv4Addr, SocketAddr, SocketAddrV4};
use std::thread;
use std::time::Duration;

// ============================================================================ 
// 示例 1: TCP 服务端基本实现
// ============================================================================ 
#[allow(dead_code)]
fn example1_tcp_server() -> io::Result<()> {
    // 绑定到本地端口 8080
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    println!("TCP 服务端监听在: 127.0.0.1:8080");

    // 接受连接
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("新连接: {:?}", stream.peer_addr()?);

                // 处理连接
                handle_client(stream)?;
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
            }
        }
    }

    Ok(())
}

#[allow(dead_code)]
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

// ============================================================================ 
// 示例 2: TCP 客户端基本实现
// ============================================================================ 
#[allow(dead_code)]
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

// ============================================================================ 
// 示例 3: TCP 回显服务器（Echo Server）
// ============================================================================ 
#[allow(dead_code)]
fn example3_echo_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8081")?;
    println!("Echo 服务端监听在: 127.0.0.1:8081");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let peer = stream.peer_addr().unwrap();
                println!("Echo 客户端连接: {}", peer);

                thread::spawn(move || {
                    if let Err(e) = handle_echo_client(stream) {
                        eprintln!("Echo 处理错误: {}", e);
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

#[allow(dead_code)]
fn handle_echo_client(stream: TcpStream) -> io::Result<()> {
    let mut writer = stream.try_clone()?;
    let reader = BufReader::new(&stream);

    for line in reader.lines() {
        match line {
            Ok(message) => {
                println!("收到: {}", message);

                // 回显消息
                writer.write_all((message + "\n").as_bytes())?;
            }
            Err(_) => break,
        }
    }

    Ok(())
}

// ============================================================================ 
// 示例 4: UDP 服务端
// ============================================================================ 
#[allow(dead_code)]
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

// ============================================================================ 
// 示例 5: UDP 客户端
// ============================================================================ 
#[allow(dead_code)]
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
// 示例 7: 非阻塞 Socket（设置超时）
// ============================================================================ 
#[allow(dead_code)]
fn example7_timeout_socket() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8083")?;

    // 设置超时 (注意：TcpListener 没有 set_timeout 方法，这里仅为演示意图，实际需在 accept 后对 stream 设置)
    // listener.set_timeout(Some(Duration::from_secs(5)))?;
    println!("TCP 监听器 (TcpListener 本身不支持直接设置 accept 超时，通常使用非阻塞模式或 select)");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("连接接受: {:?}", stream.peer_addr()?);

                // 设置读超时
                stream.set_read_timeout(Some(Duration::from_secs(3)))?;
                println!("连接设置了 3 秒读超时");

                // 简单处理
                drop(stream);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
                println!("超时等待连接");
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
                break;
            }
        }
    }

    Ok(())
}

// ============================================================================ 
// 示例 8: 多线程并发处理
// ============================================================================ 
#[allow(dead_code)]
fn example8_concurrent_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8084")?;
    println!("并发 TCP 服务端监听在: 127.0.0.1:8084");

    let mut handles = vec![];

    for stream in listener.incoming().take(5) {
        match stream {
            Ok(stream) => {
                let handle = thread::spawn(move || {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("处理错误: {}", e);
                    }
                });
                handles.push(handle);
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
            }
        }
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    Ok(())
}

// ============================================================================ 
// 示例 9: 简单的聊天服务器
// ============================================================================ 
#[allow(dead_code)]
fn example9_chat_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8085")?;
    println!("聊天服务器监听在: 127.0.0.1:8085");

    println!("聊天服务器需要多客户端支持，这里只是简单演示");
    println!("实际实现需要使用 Vec<Mutex<TcpStream>> 或类似结构");

    for stream in listener.incoming().take(1).flatten() {
        let peer = stream.peer_addr().unwrap();
        println!("聊天客户端连接: {}", peer);
        drop(stream);
    }

    Ok(())
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

// ============================================================================ 
// 示例 12: TCP 保持连接（Keep-alive）
// ============================================================================ 
#[allow(dead_code)]
fn example12_keepalive() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8086")?;

    for stream in listener.incoming().take(1).flatten() {
        // 设置 TCP Keep-alive (注: 标准库 TcpStream 暂无 set_keepalive，需用 socket2 或类似库)
        // stream.set_keepalive(Some(Duration::from_secs(60)))?;
        println!("TCP Keep-alive (标准库需额外 crate 支持)");
        drop(stream);
    }

    Ok(())
}

// ============================================================================ 
// 示例 13: TCP 无延迟（No delay）
// ============================================================================ 
#[allow(dead_code)]
fn example13_no_delay() -> io::Result<()> {
    let stream = TcpStream::connect("127.0.0.1:8080")?;

    // 设置无延迟（禁用 Nagle 算法）
    stream.set_nodelay(true)?;
    println!("TCP 无延迟已启用");

    Ok(())
}

// ============================================================================ 
// 示例 14: 检测连接断开
// ============================================================================ 
#[allow(dead_code)]
fn example14_detect_disconnect() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8087")?;

    for mut stream in listener.incoming().take(1).flatten() {
        let peer = stream.peer_addr().unwrap();
        println!("连接来自: {}", peer);

        // 尝试读取，检测连接断开
        let mut buffer = [0; 1];
        match stream.read(&mut buffer) {
            Ok(0) => {
                println!("连接已关闭（读取到 0 字节）");
            }
            Ok(n) => {
                println!("读取到 {} 字节", n);
            }
            Err(e) => {
                println!("读取错误（可能连接断开）: {}", e);
            }
        }
    }

    Ok(())
}

// ============================================================================ 
// 示例 15: 广播 UDP 消息
// ============================================================================ 
#[allow(dead_code)]
fn example15_udp_broadcast() -> io::Result<()> {
    // 创建 UDP Socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 设置广播
    socket.set_broadcast(true)?;
    println!("UDP 广播已启用");

    // 广播地址
    let broadcast_addr = "255.255.255.255:9999";
    let message = "广播消息！";

    socket.send_to(message.as_bytes(), broadcast_addr)?;
    println!("广播消息发送到: {}", broadcast_addr);

    Ok(())
}

// ============================================================================ 
// 示例 16: 多播 UDP (Multicast)
// ============================================================================ 
#[allow(dead_code)]
fn example16_udp_multicast() -> io::Result<()> {
    // 多播地址
    let multicast_addr = "239.255.255.250:1900";

    // 创建 UDP Socket
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    // 加入多播组
    let multi_addr = "239.255.255.250".parse().unwrap();
    socket.join_multicast_v4(&multi_addr, &Ipv4Addr::UNSPECIFIED)?;
    println!("已加入多播组: {}", multi_addr);

    // 发送多播消息
    socket.send_to(b"Multicast message", multicast_addr)?;
    println!("多播消息已发送");

    Ok(())
}

// ============================================================================ 
// 示例 17: TCP 性能测试（简单版）
// ============================================================================ 
#[allow(dead_code)]
fn example17_tcp_benchmark() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8088")?;

    // 服务端线程
    let handle = thread::spawn(move || {
        for mut stream in listener.incoming().take(1).flatten() {
            let mut total = 0u64;
            let mut buffer = [0; 8192];

            while let Ok(n) = stream.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                total += n as u64;
            }

            println!("总共接收: {} 字节", total);
        }
    });

    // 客户端发送数据
    thread::sleep(Duration::from_millis(100));
    let mut stream = TcpStream::connect("127.0.0.1:8088")?;

    let data = vec![0u8; 8192];
    let start = std::time::Instant::now();

    for _ in 0..1000 {
        stream.write_all(&data)?;
    }

    let duration = start.elapsed();
    let total_bytes = data.len() as u64 * 1000;
    let mbps = (total_bytes as f64 / 1024.0 / 1024.0) / duration.as_secs_f64();

    println!("发送: {} 字节，耗时: {:?}", total_bytes, duration);
    println!("速度: {:.2} MB/s", mbps);

    handle.join().unwrap();
    Ok(())
}

// ============================================================================ 
// 示例 18: 简单的代理服务器
// ============================================================================ 
#[allow(dead_code)]
fn example18_simple_proxy() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8089")?;
    println!("代理服务器监听在: 127.0.0.1:8089");

    println!("代理服务器需要转发流量到目标服务器");
    println!("这里只是简单框架");

    for _stream in listener.incoming().take(1).flatten() {
        println!("客户端连接");
        // 实际代理需要转发流量到目标服务器
    }

    Ok(())
}

// ============================================================================ 
// 示例 19: 读取大文件（分块传输）
// ============================================================================ 
#[allow(dead_code)]
fn example19_chunked_transfer() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8090")?;

    for mut stream in listener.incoming().take(1).flatten() {
        println!("分块传输开始");

        let mut buffer = [0; 4096];
        let mut total = 0u64;

        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("传输完成，总共: {} 字节", total);
                    break;
                }
                Ok(n) => {
                    total += n as u64;
                    // 处理数据...
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}

// ============================================================================ 
// 示例 20: 心跳检测（Heartbeat）
// ============================================================================ 
#[allow(dead_code)]
fn example20_heartbeat() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8091")?;

    for mut stream in listener.incoming().take(1).flatten() {
        println!("心跳检测服务器");

        stream.set_read_timeout(Some(Duration::from_secs(5)))?;

        let mut buffer = [0; 64];
        loop {
            match stream.read(&mut buffer) {
                Ok(0) => {
                    println!("客户端断开连接");
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("收到心跳: {}", message);
                    stream.write_all(b"PONG")?;
                }
                Err(e) if e.kind() == io::ErrorKind::TimedOut => {
                    println!("心跳超时");
                    break;
                }
                Err(e) => {
                    eprintln!("错误: {}", e);
                    break;
                }
            }
        }
    }

    Ok(())
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== Rust TCP/UDP 网络编程基础示例 ===\n");

    println!("示例 1: TCP 服务端基本实现");
    println!("（需要单独运行，阻塞监听）");
    println!("运行示例 2 作为客户端进行测试\n");

    println!("示例 2: TCP 客户端基本实现");
    println!("连接到 127.0.0.1:8080 发送消息\n");

    println!("示例 3: Echo 服务器");
    println!("（需要单独运行）\n");

    println!("示例 4: UDP 服务端");
    println!("（需要单独运行）\n");

    println!("示例 5: UDP 客户端");
    println!("发送 UDP 消息\n");

    println!("示例 6: SocketAddr 操作");
    example6_socket_address();
    println!();

    println!("示例 7: 设置超时");
    println!("（需要单独运行）\n");

    println!("示例 8: 多线程并发处理");
    println!("（需要单独运行）\n");

    println!("示例 9: 聊天服务器");
    println!("（需要单独运行）\n");

    println!("示例 10: 检查端口是否可用");
    example10_check_port_available(8080).unwrap();
    example10_check_port_available(9999).unwrap();
    println!();

    println!("示例 11: 获取本地 IP 地址");
    example11_local_ip();
    println!();

    println!("示例 12: TCP Keep-alive");
    println!("（需要单独运行）\n");

    println!("示例 13: TCP 无延迟");
    println!("连接到运行中的服务端\n");

    println!("示例 14: 检测连接断开");
    println!("（需要单独运行）\n");

    println!("示例 15: UDP 广播");
    println!("发送广播消息\n");

    println!("示例 16: UDP 多播");
    println!("加入多播组\n");

    println!("示例 17: TCP 性能测试");
    println!("（需要单独运行）\n");

    println!("示例 18: 简单的代理服务器");
    println!("（需要单独运行）\n");

    println!("示例 19: 分块传输");
    println!("（需要单独运行）\n");

    println!("示例 20: 心跳检测");
    println!("（需要单独运行）\n");

    println!("=== 总结 ===");
    println!("std::net 特点:");
    println!("  - TcpListener: TCP 服务端监听");
    println!("  - TcpStream: TCP 连接流");
    println!("  - UdpSocket: UDP Socket");
    println!("  - SocketAddr: 网络地址");
    println!("  - 同步 I/O，阻塞操作");
    println!("  - 基础网络编程");
    println!("  - 适合学习底层网络");
    println!("\n实际应用:");
    println!("  - HTTP 框架使用 tokio/async-std");
    println!("  - reqwest, axum, actix-web 等");
    println!("  - 异步网络性能更好");
}