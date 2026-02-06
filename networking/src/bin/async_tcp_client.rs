use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== 异步 TCP 客户端示例 ===");
    match TcpStream::connect("127.0.0.1:8080").await {
        Ok(mut stream) => {
            println!("已连接");
            stream.write_all("你好".as_bytes()).await?;
            let mut buffer = [0; 1024];
            let n = stream.read(&mut buffer).await?;
            let response = String::from_utf8_lossy(&buffer[..n]);
            println!("响应: {}", response);
        }
        Err(e) => {
            println!("无法连接到服务端: {}", e);
        }
    }
    Ok(())
}
