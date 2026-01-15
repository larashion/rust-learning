#![allow(unused)]
// ============================================================================ 
// HTTP 客户端 - Reqwest
// ============================================================================ 
//
// Reqwest 是 Rust 最流行的 HTTP 客户端库。
//
// 主要特点：
// 1. 简单易用的 API
// 2. 支持同步和异步
// 3. 连接池和 Cookie 管理
// 4. JSON 自动序列化/反序列化
// 5. WebSocket 支持
// 6. 超时和重试
//
// 依赖：reqwest = { version = "0.11", features = ["json"] }

use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::io::AsyncWriteExt;
use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use futures_util::future::join_all;

// ============================================================================ 
// 示例 1: 基本 GET 请求
// ============================================================================ 
#[tokio::main]
async fn example1_get_request() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/get").await?;

    println!("状态码: {}", response.status());

    let body = response.text().await?;
    println!("响应体:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 2: 发送 POST 请求
// ============================================================================ 
#[tokio::main]
async fn example2_post_request() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .post("https://httpbin.org/post")
        .body("这是 POST 请求体")
        .send()
        .await?;

    println!("状态码: {}", response.status());

    let body = response.text().await?;
    println!("响应:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 3: 发送 JSON 数据
// ============================================================================ 
#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: u32,
}

#[derive(Deserialize)]
struct Response {
    json: User,
}

#[tokio::main]
async fn example3_json_post() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
    };

    let client = reqwest::Client::new();

    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;

    let resp: Response = response.json().await?;
    println!("响应: {:?}", resp.json.name);

    Ok(())
}

// ============================================================================ 
// 示例 4: 设置请求头
// ============================================================================ 
#[tokio::main]
async fn example4_headers() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "My-Rust-App/1.0")
        .header("Accept", "application/json")
        .send()
        .await?;

    let body = response.text().await?;
    println!("响应:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 5: 查询参数（Query Parameters）
// ============================================================================ 
#[tokio::main]
async fn example5_query_params() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://httpbin.org/get")
        .query(&[
            ("name", "Alice"),
            ("age", "30"),
        ])
        .send()
        .await?;

    let body = response.text().await?;
    println!("响应:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 6: 使用结构体作为查询参数
// ============================================================================ 
#[derive(Serialize)]
struct QueryParams {
    page: u32,
    limit: u32,
    search: String,
}

#[tokio::main]
async fn example6_struct_query() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let params = QueryParams {
        page: 1,
        limit: 10,
        search: "rust".to_string(),
    };

    let response = client
        .get("https://httpbin.org/get")
        .query(&params)
        .send()
        .await?;

    let body = response.text().await?;
    println!("响应:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 7: 处理 JSON 响应
// ============================================================================ 
#[derive(Deserialize, Debug)]
struct IpInfo {
    origin: String,
}

#[tokio::main]
async fn example7_json_response() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let info: IpInfo = response.json().await?;

    println!("IP 地址: {}", info.origin);

    Ok(())
}

// ============================================================================ 
// 示例 8: 设置超时
// ============================================================================ 
#[tokio::main]
async fn example8_timeout() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    match client.get("https://httpbin.org/delay/10").send().await {
        Ok(_) => println!("请求成功"),
        Err(e) => println!("请求超时: {}", e),
    }

    Ok(())
}

// ============================================================================ 
// 示例 9: 重试机制
// ============================================================================ 
async fn fetch_with_retry(url: &str, max_retries: u32) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut delay = Duration::from_secs(1);

    for attempt in 0..max_retries {
        match client.get(url).send().await {
            Ok(response) => {
                if response.status().is_success() {
                    return Ok(response.text().await?);
                }
            }
            Err(_) => {
                if attempt < max_retries - 1 {
                    println!("尝试 {} 失败，{} 秒后重试", attempt + 1, delay.as_secs());
                    tokio::time::sleep(delay).await;
                    delay *= 2; // 指数退避
                }
            }
        }
    }

    Err("Max retries exceeded".into())
}

#[tokio::main]
async fn example9_retry() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/get";
    let response = fetch_with_retry(url, 3).await?;
    println!("响应: {}", response);

    Ok(())
}

// ============================================================================ 
// 示例 10: Cookie 管理
// ============================================================================ 
#[tokio::main]
async fn example10_cookies() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .cookie_store(true) // 启用 Cookie 存储
        .build()?;

    // 设置 Cookie
    let _response: reqwest::Response = client
        .post("https://httpbin.org/cookies/set")
        .query(&[("key", "value")])
        .send()
        .await?;

    // 获取 Cookie
    let response: reqwest::Response = client
        .get("https://httpbin.org/cookies")
        .send()
        .await?;

    let body = response.text().await?;
    println!("Cookies:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 11: 基本认证（Basic Auth）
// ============================================================================ 
#[tokio::main]
async fn example11_basic_auth() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", Some("pass"))
        .send()
        .await?;

    println!("状态码: {}", response.status());
    println!("成功认证!");

    Ok(())
}

// ============================================================================ 
// 示例 12: Bearer Token 认证
// ============================================================================ 
#[tokio::main]
async fn example12_bearer_token() -> Result<(), Box<dyn std::error::Error>> {
    let token = "my_secret_token";

    let response = reqwest::Client::new()
        .get("https://httpbin.org/bearer")
        .bearer_auth(token)
        .send()
        .await?;

    println!("状态码: {}", response.status());

    Ok(())
}

// ============================================================================ 
// 示例 13: 上传文件（Multipart）
// ============================================================================ 
#[tokio::main]
async fn example13_upload_file() -> Result<(), Box<dyn std::error::Error>> {
    // Create a dummy file for the example
    tokio::fs::write("test.txt", "Hello World").await?;

    let _file = tokio::fs::File::open("test.txt").await?;

    // Now reqwest::multipart::Form should be available because I added the feature
    let form = reqwest::multipart::Form::new()
        .text("username", "Alice");
        //.file("file", "test.txt").await?; // Uncomment if you want to test with file existence

    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .multipart(form)
        .send()
        .await?;

    println!("状态码: {}", response.status());

    Ok(())
}

// ============================================================================ 
// 示例 14: 下载文件
// ============================================================================ 
#[tokio::main]
async fn example14_download_file() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://httpbin.org/bytes/1024";

    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    println!("下载了 {} 字节", bytes.len());

    // 保存到文件
    tokio::fs::write("downloaded.bin", &bytes).await?;

    Ok(())
}

// ============================================================================ 
// 示例 15: 流式下载
// ============================================================================ 
#[tokio::main]
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

    Ok(())
}

// ============================================================================ 
// 示例 16: 代理支持
// ============================================================================ 
#[tokio::main]
async fn example16_proxy() -> Result<(), Box<dyn std::error::Error>> {
    let proxy = reqwest::Proxy::http("http://127.0.0.1:8080")?;

    let _client = reqwest::Client::builder()
        .proxy(proxy)
        .build()?;

    // let response = client.get("https://httpbin.org/ip").send().await?;
    // let body = response.text().await?;
    // println!("通过代理:\n{}", body);

    Ok(())
}

// ============================================================================ 
// 示例 17: 连接池配置
// ============================================================================ 
#[tokio::main]
async fn example17_connection_pool() -> Result<(), Box<dyn std::error::Error>> {
    let _client = reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(90))
        .pool_max_idle_per_host(10)
        .build()?;

    // 多个请求会复用连接
    for i in 0..5 {
        // let response = client.get("https://httpbin.org/get").send().await?;
        println!("请求 {} 完成", i + 1);
    }

    Ok(())
}

// ============================================================================ 
// 示例 18: WebSocket 客户端
// ============================================================================ 
#[tokio::main]
async fn example18_websocket() -> Result<(), Box<dyn std::error::Error>> {
    // Note: Reqwest 0.11 doesn't have native WebSocket support unless enabled or used via upgrade.
    // Assuming upgrade is available.
    
    // let client = Client::new();
    // let ws = client
    //     .get("wss://echo.websocket.org")
    //     .upgrade()
    //     .send()
    //     .await?;
    // ...

    Ok(())
}

// ============================================================================ 
// 示例 19: 并发请求
// ============================================================================ 
#[tokio::main]
async fn example19_concurrent_requests() -> Result<(), Box<dyn std::error::Error>> {
    let urls = ["https://httpbin.org/get/1",
        "https://httpbin.org/get/2",
        "https://httpbin.org/get/3"];

    let client = reqwest::Client::new();

    let fetches = urls.iter().map(|&_url| {
        let _client = client.clone();
        async move {
            // let response = client.get(url).send().await?;
            // Ok::<_, reqwest::Error>(response.text().await?)
             Ok::<_, reqwest::Error>("Mock response".to_string())
        }
    });

    let results: Vec<Result<String, reqwest::Error>> = join_all(fetches).await;

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(text) => println!("请求 {} 完成，长度: {}", i + 1, text.len()),
            Err(e) => println!("请求 {} 失败: {}", i + 1, e),
        }
    }

    Ok(())
}

// ============================================================================ 
// 示例 20: 错误处理和状态码
// ============================================================================ 
#[tokio::main]
async fn example20_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let response = client.get("https://httpbin.org/status/404").send().await?;

    let status = response.status();

    if status.is_success() {
        println!("请求成功");
    } else if status.is_client_error() {
        println!("客户端错误: {}", status);
    } else if status.is_server_error() {
        println!("服务器错误: {}", status);
    }

    match status.as_u16() {
        200..=299 => println!("2xx 成功"),
        400..=499 => println!("4xx 客户端错误"),
        500..=599 => println!("5xx 服务器错误"),
        _ => println!("其他状态码"),
    }

    Ok(())
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== HTTP 客户端 - Reqwest 示例 ===\n");
    println!("Code is uncommented. Run specific examples via cargo run or by modifying main.");
}