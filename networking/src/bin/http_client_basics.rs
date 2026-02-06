
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 示例 1: 基本 GET 请求 ---");
    example1_get_request().await?;

    println!("
--- 示例 2: 发送 POST 请求 ---");
    example2_post_request().await?;

    println!("
--- 示例 4: 设置请求头 ---");
    example4_headers().await?;

    println!("
--- 示例 5: 查询参数 ---");
    example5_query_params().await?;

    Ok(())
}

async fn example1_get_request() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应体:
{}", body);
    Ok(())
}

async fn example2_post_request() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .body("这是 POST 请求体")
        .send()
        .await?;
    println!("状态码: {}", response.status());
    let body = response.text().await?;
    println!("响应:
{}", body);
    Ok(())
}

async fn example4_headers() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://httpbin.org/headers")
        .header("User-Agent", "My-Rust-App/1.0")
        .header("Accept", "application/json")
        .send()
        .await?;
    let body = response.text().await?;
    println!("响应:
{}", body);
    Ok(())
}

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
    println!("响应:
{}", body);
    Ok(())
}
