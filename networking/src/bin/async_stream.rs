use tokio_stream::StreamExt;

#[tokio::main]
async fn main() {
    println!("=== 异步迭代器 (Stream) 示例 ===");
    let mut stream = tokio_stream::iter(vec![1, 2, 3, 4, 5]);
    while let Some(value) = stream.next().await {
        println!("异步迭代: {}", value);
    }
}
