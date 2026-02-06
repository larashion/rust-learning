use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("=== 简单异步 HTTP 请求示例 ===");
    // 注意：example.com 可能重定向到 https，这里仅演示最基础的 HTTP/1.1
    if let Ok(mut stream) = TcpStream::connect("example.com:80").await {
        let request = b"GET / HTTP/1.1
Host: example.com
Connection: close

";
        stream.write_all(request).await?;
        
        let mut response = Vec::new();
        stream.read_to_end(&mut response).await?;
        
        println!("响应:
{}", String::from_utf8_lossy(&response));
    } else {
        println!("无法连接到 example.com");
    }
    Ok(())
}
