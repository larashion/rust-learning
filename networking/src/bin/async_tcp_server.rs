use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== 异步 TCP 服务端示例 ===");
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("监听在: 127.0.0.1:8080");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("新连接: {}", addr);

        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            loop {
                match socket.read(&mut buffer).await {
                    Ok(0) => return, // 连接关闭
                    Ok(n) => {
                        let message = String::from_utf8_lossy(&buffer[..n]);
                        println!("收到: {}", message);
                        if let Err(e) = socket.write_all("异步响应".as_bytes()).await {
                             eprintln!("写入错误: {}", e);
                             return;
                        }
                    }
                    Err(e) => {
                        eprintln!("读取错误: {}", e);
                        return;
                    }
                }
            }
        });
    }
}
