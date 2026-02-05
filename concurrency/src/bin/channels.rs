// ============================================================================
// 通道 (Channels) - 消息传递并发
// ============================================================================
//
// Rust 使用 std::sync::mpsc 提供多生产者单消费者通道。
//
// 主要特点：
// 1. "不要通过共享内存来通讯，而要通过通讯来共享内存"（Go 语言的哲学）
// 2. mpsc: multiple producer, single consumer
// 3. 类型安全的消息传递
// 4. 通道所有权分离：Sender 可以克隆，Receiver 不能
// 5. send() 返回 Result，可以处理错误

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// ============================================================================
// 示例 1: 基本的消息传递
// ============================================================================
fn example1_basic_channel() {
    // 创建通道
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("你好");
        // 发送消息
        tx.send(val).unwrap();
        // val 的所有权已被转移
        // println!("{}", val); // 编译错误！
    });

    // 接收消息（阻塞）
    let received = rx.recv().unwrap();
    println!("接收到: {}", received);
}

// ============================================================================
// 示例 2: 发送多个消息
// ============================================================================
fn example2_multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..=5 {
            tx.send(i).unwrap();
            println!("发送: {}", i);
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 接收所有消息
    for received in rx {
        println!("接收: {}", received);
    }
}

// ============================================================================
// 示例 3: 多个生产者
// ============================================================================
fn example3_multiple_producers() {
    let (tx, rx) = mpsc::channel();

    // 克隆 Sender，创建多个生产者
    let tx1 = tx.clone();
    let tx2 = tx.clone();

    let handle1 = thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("生产者1: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let handle2 = thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("生产者2: {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // 原始的 tx 也被 drop
    drop(tx);

    handle1.join().unwrap();
    handle2.join().unwrap();

    // 接收所有消息
    println!("接收到的消息:");
    for received in rx {
        println!("  {}", received);
    }
}

// ============================================================================
// 示例 4: try_recv - 非阻塞接收
// ============================================================================
fn example4_try_recv() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        tx.send(42).unwrap();
    });

    println!("尝试非阻塞接收...");
    match rx.try_recv() {
        Ok(value) => println!("接收到: {}", value),
        Err(e) => println!("暂无消息: {:?}", e),
    }

    thread::sleep(Duration::from_millis(250));

    println!("再次尝试...");
    match rx.try_recv() {
        Ok(value) => println!("接收到: {}", value),
        Err(e) => println!("错误: {:?}", e),
    }
}

// ============================================================================
// 示例 5: recv_timeout - 带超时的接收
// ============================================================================
fn example5_recv_timeout() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("延迟消息".to_string()).unwrap();
    });

    println!("等待消息（超时 50ms）...");
    match rx.recv_timeout(Duration::from_millis(50)) {
        Ok(msg) => println!("接收到: {}", msg),
        Err(_) => println!("超时！"),
    }

    println!("等待消息（超时 200ms）...");
    match rx.recv_timeout(Duration::from_millis(200)) {
        Ok(msg) => println!("接收到: {}", msg),
        Err(_) => println!("超时！"),
    }
}

// ============================================================================
// 示例 6: 发送不同类型的消息
// ============================================================================
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn example6_enum_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(Message::Move { x: 10, y: 20 }).unwrap();
        tx.send(Message::Write(String::from("你好"))).unwrap();
        tx.send(Message::ChangeColor(255, 0, 0)).unwrap();
        tx.send(Message::Quit).unwrap();
    });

    println!("处理不同类型的消息:");
    for msg in rx {
        match msg {
            Message::Quit => {
                println!("  收到退出消息");
                break;
            }
            Message::Move { x, y } => {
                println!("  移动到 ({}, {})", x, y);
            }
            Message::Write(text) => {
                println!("  写入: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("  颜色: ({}, {}, {})", r, g, b);
            }
        }
    }
}

// ============================================================================
// 示例 7: 无界通道 vs 有界通道
// ============================================================================
fn example7_channel_capacity() {
    // 标准 mpsc::channel 是无界通道
    // 如果需要限制缓冲区大小，可以使用 crossbeam 或 tokio 的通道

    // 这里展示标准通道的行为
    let (tx, rx) = mpsc::channel();

    let producer = thread::spawn(move || {
        for i in 0..100 {
            tx.send(i).unwrap();
            // 无界通道不会阻塞
        }
        println!("生产者完成");
    });

    thread::sleep(Duration::from_millis(10)); // 让生产者先运行

    println!("消费者开始接收:");
    for received in rx.iter().take(10) {
        println!("  接收: {}", received);
    }
    // 后续消息在 rx 被 drop 后丢失

    producer.join().unwrap();
}

// ============================================================================
// 示例 8: 同步通道 (rendezvous channel)
// ============================================================================
fn example8_sync_channel() {
    // 同步通道在发送时会阻塞，直到有接收者
    let (tx, rx) = mpsc::sync_channel(0); // 缓冲区大小为 0

    let sender = thread::spawn(move || {
        println!("发送者准备发送...");
        tx.send(42).unwrap(); // 这里会阻塞，直到有接收者
        println!("发送者发送成功！");
    });

    thread::sleep(Duration::from_millis(100));

    println!("接收者准备接收...");
    let value = rx.recv().unwrap();
    println!("接收者接收到: {}", value);

    sender.join().unwrap();
}

// ============================================================================
// 示例 9: 带缓冲区的同步通道
// ============================================================================
fn example9_buffered_sync_channel() {
    // 缓冲区大小为 2
    let (tx, rx) = mpsc::sync_channel(2);

    thread::spawn(move || {
        // 前两个消息不会被阻塞
        tx.send(1).unwrap();
        println!("发送 1");
        tx.send(2).unwrap();
        println!("发送 2");

        // 第三个消息会被阻塞，直到有接收者
        println!("尝试发送 3（会阻塞）...");
        tx.send(3).unwrap();
        println!("发送 3");
    });

    thread::sleep(Duration::from_millis(100));
    println!("接收者开始接收:");
    println!("  接收: {}", rx.recv().unwrap());
    thread::sleep(Duration::from_millis(100));
    println!("  接收: {}", rx.recv().unwrap());
    println!("  接收: {}", rx.recv().unwrap());
}

// ============================================================================
// 示例 10: 通道关闭检测
// ============================================================================
fn example10_detect_closed() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 0..5 {
            if tx.send(i).is_err() {
                println!("发送者: 通道已关闭，无法发送 {}", i);
                break;
            }
            println!("发送者: 发送 {}", i);
        }
        println!("发送者完成");
    });

    thread::sleep(Duration::from_millis(100));

    // 接收前两个消息
    for _ in 0..2 {
        if let Ok(value) = rx.recv() {
            println!("接收者: 接收 {}", value);
        }
    }

    // 丢弃 rx，关闭通道
    drop(rx);
    println!("接收者: 关闭通道");

    thread::sleep(Duration::from_millis(100));
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Rust 通道 (Channels) 示例 ===\n");

    println!("示例 1: 基本的消息传递");
    example1_basic_channel();
    println!();

    println!("示例 2: 发送多个消息");
    example2_multiple_messages();
    println!();

    println!("示例 3: 多个生产者");
    example3_multiple_producers();
    println!();

    println!("示例 4: try_recv - 非阻塞接收");
    example4_try_recv();
    println!();

    println!("示例 5: recv_timeout - 带超时的接收");
    example5_recv_timeout();
    println!();

    println!("示例 6: 发送不同类型的消息");
    example6_enum_messages();
    println!();

    println!("示例 7: 无界通道 vs 有界通道");
    example7_channel_capacity();
    println!();

    println!("示例 8: 同步通道 (rendezvous)");
    example8_sync_channel();
    println!();

    println!("示例 9: 带缓冲区的同步通道");
    example9_buffered_sync_channel();
    println!();

    println!("示例 10: 通道关闭检测");
    example10_detect_closed();

    println!("\n=== 总结 ===");
    println!("Rust 通道特点:");
    println!("  - mpsc: 多生产者单消费者");
    println!("  - 类型安全的消息传递");
    println!("  - send() 会转移所有权");
    println!("  - recv() 阻塞接收，try_recv() 非阻塞");
    println!("  - recv_timeout() 带超时");
    println!("  - 支持 sync_channel（同步/有界通道）");
    println!("  - 检测通道关闭（send 返回 Err）");
    println!("  - 遵循 '通讯来共享内存' 的哲学");
}
