use std::time::Duration;
use futures_util::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 示例 8: 设置超时 ---");
    example8_timeout().await?;

    println!("
--- 示例 9: 重试机制 ---");
    example9_retry().await?;

    println!("
--- 示例 19: 并发请求 ---");
    example19_concurrent_requests().await?;

    println!("
--- 示例 20: 错误处理和状态码 ---");
    example20_error_handling().await?;

    Ok(())
}

async fn example8_timeout() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()?;
    match client.get("https://httpbin.org/delay/5").send().await {
        Ok(_) => println!("请求成功"),
        Err(e) => println!("请求超时 (预期): {}", e),
    }
    Ok(())
}

async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut delay = Duration::from_millis(500);
    for attempt in 0..max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response.text().await?);
                }
            }
            Err(_) => {
                if attempt < max_retries - 1 {
                    println!("尝试 {} 失败，正在重试...", attempt + 1);
                    tokio::time::sleep(delay).await;
                    delay *= 2;
                }
            }
        }
    }
    Err("Max retries exceeded".into())
}

async fn example9_retry() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/get";
    let response = fetch_with_retry(url, 2).await?;
    println!("重试抓取成功，响应长度: {}", response.len());
    Ok(())
}

async fn example19_concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    let urls = ["https://httpbin.org/get", "https://httpbin.org/ip", "https://httpbin.org/user-agent"];
    let client = reqwest::Client::new();
    let fetches = urls.iter().map(|&url| {
        let client = client.clone();
        async move {
            let resp = client.get(url).send().await?;
            resp.text().await
        }
    });
    let results = join_all(fetches).await;
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(text) => println!("请求 {} 完成，长度: {}", i + 1, text.len()),
            Err(e) => println!("请求 {} 失败: {}", i + 1, e),
        }
    }
    Ok(())
}

async fn example20_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get("https://httpbin.org/status/404").send().await?;
    let status = response.status();
    println!("状态码: {}", status);
    if status.is_client_error() {
        println!("检测到客户端错误: {}", status);
    }
    Ok(())
}
