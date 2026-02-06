use tokio::io::AsyncWriteExt;
use futures_util::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 示例 13: 上传文件 (Multipart) ---");
    example13_upload_file().await?;

    println!("
--- 示例 14: 下载文件 ---");
    example14_download_file().await?;

    println!("
--- 示例 15: 流式下载 ---");
    example15_streaming_download().await?;

    Ok(())
}

async fn example13_upload_file() -> Result<(), Box<dyn std::error::Error>> {
    tokio::fs::write("test.txt", "Hello World").await?;
    let form = reqwest::multipart::Form::new()
        .text("username", "Alice");
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;
    println!("上传状态码: {}", response.status());
    tokio::fs::remove_file("test.txt").await?;
    Ok(())
}

async fn example14_download_file() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/bytes/1024";
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;
    println!("下载了 {} 字节", bytes.len());
    tokio::fs::write("downloaded.bin", &bytes).await?;
    tokio::fs::remove_file("downloaded.bin").await?;
    Ok(())
}

async fn example15_streaming_download() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/bytes/1024";
    let response = reqwest::get(url).await?;
    let mut file = tokio::fs::File::create("downloaded_stream.bin").await?;
    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }
    println!("流式下载完成");
    tokio::fs::remove_file("downloaded_stream.bin").await?;
    Ok(())
}
