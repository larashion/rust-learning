use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
}

#[derive(Deserialize, Debug)]
struct PostResponse {
    json: User,
}

#[derive(Deserialize, Debug)]
struct IpInfo {
    origin: String,
}

#[derive(Serialize)]
struct QueryParams {
    page: u32,
    limit: u32,
    search: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("--- 示例 3: 发送 JSON 数据 ---");
    example3_json_post().await?;

    println!("
--- 示例 6: 使用结构体作为查询参数 ---");
    example6_struct_query().await?;

    println!("
--- 示例 7: 处理 JSON 响应 ---");
    example7_json_response().await?;

    Ok(())
}

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

    let resp: PostResponse = response.json().await?;
    println!("响应 User Name: {:?}", resp.json.name);
    Ok(())
}

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
    println!("响应:
{}", body);
    Ok(())
}

async fn example7_json_response() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://httpbin.org/ip").await?;
    let info: IpInfo = response.json().await?;
    println!("IP 地址: {}", info.origin);
    Ok(())
}
