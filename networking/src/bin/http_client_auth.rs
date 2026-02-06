
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 示例 11: 基本认证 (Basic Auth) ---");
    example11_basic_auth().await?;

    println!("
--- 示例 12: Bearer Token 认证 ---");
    example12_bearer_token().await?;

    println!("
--- 示例 10: Cookie 管理 ---");
    example10_cookies().await?;

    Ok(())
}

async fn example11_basic_auth() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", Some("pass"))
        .send()
        .await?;
    println!("状态码: {}", response.status());
    if response.status().is_success() {
        println!("成功认证!");
    }
    Ok(())
}

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

async fn example10_cookies() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .build()?;

    // 设置 Cookie
    client
        .post("https://httpbin.org/cookies/set")
        .query(&[("key", "value")])
        .send()
        .await?;

    // 获取 Cookie
    let response = client
        .get("https://httpbin.org/cookies")
        .send()
        .await?;

    let body = response.text().await?;
    println!("Cookies:
{}", body);
    Ok(())
}
