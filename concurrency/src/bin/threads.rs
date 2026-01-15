// ============================================================================
// 线程 (Threads) - Rust 并发基础
// ============================================================================
//
// Rust 使用 `std::thread` 模块提供线程支持。
//
// 主要特点：
// 1. 轻量级线程（类似 goroutine，但需要手动管理）
// 2. `spawn` 创建新线程
// 3. 线程所有权通过闭包参数传递（move）
// 4. `join()` 等待线程完成
// 5. panic 在线程间不会传播

use std::thread;
use std::time::{Duration, Instant};

// ============================================================================
// 示例 1: 创建和启动线程
// ============================================================================
fn example1_spawn_thread() {
    println!("主线程开始");

    // 创建新线程
    let handle = thread::spawn(|| {
        println!("新线程开始");
        // 线程执行一些工作
        for i in 1..=5 {
            println!("新线程: 计数 {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        println!("新线程结束");
    });

    println!("主线程继续执行");

    // 等待新线程完成
    handle.join().unwrap();

    println!("主线程结束");
}

// ============================================================================
// 示例 2: 闭包中的 move 关键字
// ============================================================================
fn example2_move_closure() {
    let v = vec![1, 2, 3];

    println!("主线程: {:?}", v);

    // 使用 move 关键字将 v 的所有权转移给线程
    let handle = thread::spawn(move || {
        println!("新线程接收: {:?}", v);
        // v 在这里被 drop
    });

    // v 的所有权已被转移，不能在这里使用
    // println!("{:?}", v); // 编译错误！

    handle.join().unwrap();
}

// ============================================================================
// 示例 3: 返回值
// ============================================================================
fn example3_return_value() {
    let handle = thread::spawn(|| {
        // 线程可以返回值
        let result: i32 = (1..=100).sum();
        result
    });

    // join 返回 Result<T, Box<dyn Any>>
    let result = handle.join().unwrap();
    println!("线程返回值: {}", result);
}

// ============================================================================
// 示例 4: 线程 panic 处理
// ============================================================================
fn example4_panic_handling() {
    let handle = thread::spawn(|| {
        panic!("线程发生了 panic!");
    });

    // join 返回 Err，包含 panic 的信息
    match handle.join() {
        Ok(_) => println!("线程正常完成"),
        Err(e) => println!("线程 panic: {:?}", e),
    }
}

// ============================================================================
// 示例 5: 多个线程并发
// ============================================================================
fn example5_multiple_threads() {
    let mut handles = vec![];

    let start = Instant::now();

    // 创建 5 个线程
    for i in 0..5 {
        let handle = thread::spawn(move || {
            let id = i;
            println!("线程 {} 开始", id);
            thread::sleep(Duration::from_millis(500));
            println!("线程 {} 完成", id);
            id * 10
        });
        handles.push(handle);
    }

    // 收集所有线程的返回值
    let mut results = vec![];
    for handle in handles {
        results.push(handle.join().unwrap());
    }

    let duration = start.elapsed();
    println!("所有线程完成，耗时: {:?}", duration);
    println!("结果: {:?}", results);
}

// ============================================================================
// 示例 6: 线程命名
// ============================================================================
fn example6_thread_name() {
    // 可以给线程命名，方便调试
    let handle = thread::Builder::new()
        .name("工作线程".to_string())
        .spawn(|| {
            // 获取当前线程的名称
            if let Some(name) = thread::current().name() {
                println!("线程名称: {}", name);
            }
        })
        .unwrap();

    handle.join().unwrap();
}

// ============================================================================
// 示例 7: 线程栈大小
// ============================================================================
fn example7_stack_size() {
    // 可以自定义线程栈大小
    let handle = thread::Builder::new()
        .stack_size(1024 * 1024) // 1MB 栈
        .spawn(|| {
            println!("自定义栈大小的线程");
        })
        .unwrap();

    handle.join().unwrap();
}

// ============================================================================
// 示例 8: 获取线程 ID
// ============================================================================
fn example8_thread_id() {
    let main_id = thread::current().id();
    println!("主线程 ID: {:?}", main_id);

    let handle = thread::spawn(move || {
        let child_id = thread::current().id();
        println!("子线程 ID: {:?}", child_id);
        child_id
    });

    let child_id = handle.join().unwrap();
    println!("主线程 ID (再次): {:?}", main_id);
    println!("子线程 ID (来自返回值): {:?}", child_id);
}

// ============================================================================
// 示例 9: 线程局部存储 (TLS)
// ============================================================================
use std::cell::RefCell;

thread_local! {
    static THREAD_LOCAL: RefCell<i32> = const { RefCell::new(0) };
}

fn example9_thread_local_storage() {
    THREAD_LOCAL.with(|val| {
        *val.borrow_mut() = 42;
    });

    let handle = thread::spawn(|| {
        // 每个线程都有自己的 THREAD_LOCAL 副本
        THREAD_LOCAL.with(|val| {
            println!("线程局部值: {}", *val.borrow());
            *val.borrow_mut() = 100;
        });
        THREAD_LOCAL.with(|val| {
            println!("修改后的值: {}", *val.borrow());
        });
    });

    handle.join().unwrap();

    // 主线程的 THREAD_LOCAL 不受影响
    THREAD_LOCAL.with(|val| {
        println!("主线程的值: {}", *val.borrow());
    });
}

// ============================================================================
// 示例 10: 作用域线程 (scoped threads)
// ============================================================================
fn example10_scoped_threads() {
    let data = vec![1, 2, 3, 4, 5];

    // 使用 scoped 线程，可以借用外部变量而不需要所有权转移
    thread::scope(|s| {
        for i in 0..data.len() {
            let data = &data;
            s.spawn(move || {
                println!("线程 {}: 数据[{}] = {}", i, i, data[i]);
            });
        }
    });

    // data 仍然有效
    println!("原始数据: {:?}", data);
}

// ============================================================================
// 示例 11: 生产者-消费者模式（基础版）
// ============================================================================
fn example11_producer_consumer() {
    // 这个是简化版，后面会用 channel 实现完整版
    let (sender, receiver) = std::sync::mpsc::channel();

    let producer = thread::spawn(move || {
        for i in 1..=5 {
            println!("生产者发送: {}", i);
            sender.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    let consumer = thread::spawn(move || {
        for received in receiver {
            println!("消费者接收: {}", received);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Rust 线程示例 ===\n");

    println!("示例 1: 创建和启动线程");
    example1_spawn_thread();
    println!();

    println!("示例 2: 闭包中的 move 关键字");
    example2_move_closure();
    println!();

    println!("示例 3: 返回值");
    example3_return_value();
    println!();

    println!("示例 4: 线程 panic 处理");
    example4_panic_handling();
    println!();

    println!("示例 5: 多个线程并发");
    example5_multiple_threads();
    println!();

    println!("示例 6: 线程命名");
    example6_thread_name();
    println!();

    println!("示例 7: 线程栈大小");
    example7_stack_size();
    println!();

    println!("示例 8: 获取线程 ID");
    example8_thread_id();
    println!();

    println!("示例 9: 线程局部存储");
    example9_thread_local_storage();
    println!();

    println!("示例 10: 作用域线程");
    example10_scoped_threads();
    println!();

    println!("示例 11: 生产者-消费者模式（基础版）");
    example11_producer_consumer();

    println!("\n=== 总结 ===");
    println!("Rust 线程特点:");
    println!("  - 使用 thread::spawn 创建线程");
    println!("  - move 关键字转移所有权");
    println!("  - join() 等待线程完成并获取返回值");
    println!("  - panic 不会跨线程传播");
    println!("  - 线程间安全通信需要使用 channel 或共享内存");
    println!("  - 支持线程命名、自定义栈大小");
    println!("  - scoped threads 允许借用外部变量");
}
