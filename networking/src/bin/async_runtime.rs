// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================ 
// 异步运行时 - Tokio 基础
// ============================================================================ 
//
// Tokio 是 Rust 最流行的异步运行时。
//
// 依赖: tokio = { version = "1", features = ["full"] }

use std::sync::Arc;
use std::io;
use std::pin::Pin;
use std::future::Future;
use tokio::sync::{Mutex, RwLock, Barrier, Semaphore, broadcast, OnceCell, mpsc};
use tokio::time::{sleep, Duration, timeout};
use tokio::fs::{self, File};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_stream::StreamExt;

// ============================================================================ 
// 示例 1: 基本 async 函数
// ============================================================================ 
#[allow(dead_code)]
async fn example1_basic_async() {
    println!("开始异步任务");
    async fn say_hello() {
        println!("你好！");
    }
    say_hello().await;
    println!("异步任务完成");
}

#[allow(dead_code)]
#[tokio::main]
async fn example2_tokio_main() {
    println!("Tokio 运行时已启动");
}

#[allow(dead_code)]
#[tokio::main]
async fn example3_spawn() {
    println!("主任务");
    let task = tokio::spawn(async {
        println!("任务 1 开始");
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        println!("任务 1 完成");
    });
    let task2 = tokio::spawn(async {
        println!("任务 2 开始");
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        println!("任务 2 完成");
    });
    println!("主任务继续");
    task.await.unwrap();
    task2.await.unwrap();
    println!("所有任务完成");
}

#[allow(dead_code)]
#[tokio::main]
async fn example4_timer() {
    println!("开始计时");
    sleep(Duration::from_secs(1)).await;
    println!("1 秒后");
    let mut interval = tokio::time::interval(Duration::from_secs(1));
    for i in 0..3 {
        interval.tick().await;
        println!("定时器 tick {}", i + 1);
    }
    match tokio::time::timeout(
        Duration::from_secs(2),
        sleep(Duration::from_secs(3))
    ).await {
        Ok(_) => println!("任务完成"),
        Err(_) => println!("任务超时"),
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn example5_channel() {
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        for i in 1..=5 {
            tx.send(i).await.unwrap();
            println!("发送: {}", i);
        }
    });
    while let Some(msg) = rx.recv().await {
        println!("接收: {}", msg);
    }
    println!("通道关闭");
}

#[allow(dead_code)]
#[tokio::main]
async fn example6_async_mutex() {
    let data = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let mut num = data.lock().await;
            *num += 1;
            println!("计数: {}", *num);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
    println!("最终计数: {}", *data.lock().await);
}

async fn task1() -> i32 {
    sleep(Duration::from_secs(1)).await;
    1
}
async fn task2() -> i32 {
    sleep(Duration::from_secs(2)).await;
    2
}
#[allow(dead_code)]
#[tokio::main]
async fn example7_join() {
    let (result1, result2) = tokio::join!(task1(), task2());
    println!("结果: {} {}", result1, result2);
}

#[allow(dead_code)]
#[tokio::main]
async fn example8_select() {
    let (tx1, mut rx1) = mpsc::channel(10);
    let (tx2, mut rx2) = mpsc::channel(10);
    tokio::spawn(async move {
        sleep(Duration::from_secs(1)).await;
        tx1.send("来自通道 1").await.unwrap();
    });
    tokio::spawn(async move {
        sleep(Duration::from_secs(2)).await;
        tx2.send("来自通道 2").await.unwrap();
    });
    tokio::select! {
        msg = rx1.recv() => {
            println!("通道 1 收到: {:?}", msg);
        }
        msg = rx2.recv() => {
            println!("通道 2 收到: {:?}", msg);
        }
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn example9_async_io() -> io::Result<()> {
    // 异步写入文件
    let mut file = File::create("async_test.txt").await?;
    file.write_all("异步文件内容".as_bytes()).await?;
    file.flush().await?;

    // 异步读取文件
    let mut content = String::new();
    let mut file = File::open("async_test.txt").await?;
    file.read_to_string(&mut content).await?;

    println!("内容: {}", content);

    // 清理
    fs::remove_file("async_test.txt").await?;

    Ok(())
}

// ============================================================================ 
// 示例 10: 异步 TCP 服务端
// ============================================================================ 
#[allow(dead_code)]
#[tokio::main]
async fn example10_async_tcp_server() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("异步 TCP 服务端监听在: 127.0.0.1:8080");

    // 仅接受一次连接演示，避免死循环阻塞
    if let Ok((mut socket, addr)) = listener.accept().await {
        println!("新连接: {}", addr);

        // 为每个连接创建新任务
        tokio::spawn(async move {
            let mut buffer = [0; 1024];
            match socket.read(&mut buffer).await {
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("收到: {}", message);

                    socket.write_all("异步响应".as_bytes()).await.unwrap();
                }
                Err(e) => {
                    eprintln!("读取错误: {}", e);
                }
            }
        });
    }
    Ok(())
}

// ============================================================================ 
// 示例 11: 异步 TCP 客户端
// ============================================================================ 
#[allow(dead_code)]
#[tokio::main]
async fn example11_async_tcp_client() -> io::Result<()> {
    // 尝试连接，如果失败则忽略
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8080").await {
        println!("已连接");

        // 发送消息
        stream.write_all("你好".as_bytes()).await?;

        // 读取响应
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        let response = String::from_utf8_lossy(&buffer[..n]);
        println!("响应: {}", response);
    } else {
        println!("无法连接到服务端 (可能未启动)");
    }

    Ok(())
}

#[allow(dead_code)]
#[tokio::main]
async fn example12_barrier() {
    let barrier = Arc::new(Barrier::new(3));
    let mut handles = vec![];
    for i in 0..3 {
        let barrier = Arc::clone(&barrier);
        let handle = tokio::spawn(async move {
            println!("任务 {} 准备中", i);
            barrier.wait().await;
            println!("任务 {} 继续", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn example13_semaphore() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut handles = vec![];
    for i in 0..10 {
        let semaphore = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            println!("任务 {} 开始", i);
            tokio::time::sleep(Duration::from_secs(1)).await;
            println!("任务 {} 完成", i);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
}

#[allow(dead_code)]
#[tokio::main]
async fn example14_broadcast() {
    let (tx, mut rx1) = broadcast::channel(10);
    let mut rx2 = tx.subscribe();
    tokio::spawn(async move {
        for i in 1..=3 {
            tx.send(i).unwrap();
            println!("广播: {}", i);
        }
    });
    tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            println!("接收者 1: {}", msg);
        }
    });
    tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            println!("接收者 2: {}", msg);
        }
    });
    tokio::time::sleep(Duration::from_secs(1)).await;
}

#[allow(dead_code)]
#[tokio::main]
async fn example15_rwlock() {
    let data = Arc::new(RwLock::new(0));
    let mut handles = vec![];
    for i in 0..3 {
        let data = Arc::clone(&data);
        let handle = tokio::spawn(async move {
            let r = data.read().await;
            println!("读者 {}: {}", i, *r);
        });
        handles.push(handle);
    }
    let writer_data = Arc::clone(&data);
    let handle = tokio::spawn(async move {
        let mut w = writer_data.write().await;
        *w = 100;
        println!("写者: 更新为 {}", *w);
    });
    handles.push(handle);
    for handle in handles {
        handle.await.unwrap();
    }
    println!("最终值: {}", *data.read().await);
}

static CONFIG: OnceCell<String> = OnceCell::const_new();
#[allow(dead_code)]
#[tokio::main]
async fn example16_oncecell() {
    let value = CONFIG.get_or_init(|| async {
        println!("初始化配置...");
        "配置值".to_string()
    }).await;
    println!("配置: {}", value);
    let value2 = CONFIG.get_or_init(|| async {
        "不会执行".to_string()
    }).await;
    println!("配置2: {}", value2);
}

#[allow(dead_code)]
#[tokio::main]
async fn example17_async_iter() {
    let mut stream = tokio_stream::iter(vec![1, 2, 3, 4, 5]);
    while let Some(value) = stream.next().await {
        println!("异步迭代: {}", value);
    }
}

async fn unreliable_operation() -> Result<i32, &'static str> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    // if rand::random() {
    if true { // Simplified for compilation
        Ok(42)
    } else {
        Err("随机失败")
    }
}

async fn retry_with_backoff<F, T, E>(
    mut operation: F,
    max_retries: usize,
    initial_delay: Duration
) -> Result<T, E>
where
    F: FnMut() -> Pin<Box<dyn Future<Output = Result<T, E>> + Send>>,
{
    let mut delay = initial_delay;
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) if attempt < max_retries => {
                println!("尝试 {} 失败，{} 秒后重试", attempt + 1, delay.as_secs());
                tokio::time::sleep(delay).await;
                delay *= 2;
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!()
}

#[allow(dead_code)]
#[tokio::main]
async fn example18_retry() {
    match timeout(Duration::from_secs(5), unreliable_operation()).await {
        Ok(Ok(result)) => println!("成功: {}", result),
        Ok(Err(e)) => println!("失败: {}", e),
        Err(_) => println!("超时"),
    }
}

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};

#[cfg(unix)]
#[allow(dead_code)]
#[tokio::main]
async fn example19_signal() {
    let mut sigterm = signal(SignalKind::terminate()).unwrap();
    println!("等待 SIGTERM 信号...");
    sigterm.recv().await;
    println!("收到 SIGTERM，优雅关闭");
}

#[cfg(windows)]
#[allow(dead_code)]
#[tokio::main]
async fn example19_signal() {
    println!("Unix 信号处理仅在 Linux/Mac 上可用");
}

#[allow(dead_code)]
#[tokio::main]
async fn example20_simple_http() -> io::Result<()> {
    if let Ok(mut stream) = TcpStream::connect("example.com:80").await {
        let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
        stream.write_all(request).await?;
        let mut response = vec![0u8; 4096];
        let n = stream.read(&mut response).await?;
        println!("响应:\n{}", String::from_utf8_lossy(&response[..n]));
    }
    Ok(())
}

fn main() {
    println!("=== Tokio 异步运行时示例 ===\n");
    println!("所有示例代码已取消注释并编译。");
}