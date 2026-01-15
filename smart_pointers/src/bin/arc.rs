// ============================================================================
// Arc<T> - 原子引用计数智能指针
// ============================================================================
//
// Arc<T> (Atomic Reference Counted) 是 Rc<T> 的线程安全版本。
//
// 主要特点：
// 1. 类似 Rc，允许共享所有权
// 2. 引用计数使用原子操作，线程安全
// 3. 实现了 Send 和 Sync，可以在多线程间共享
// 4. 性能略低于 Rc（原子操作开销）

use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// 示例 1: 基本用法 - 跨线程共享不可变数据
// ============================================================================
fn example1_basic_arc() {
    // 创建 Arc，在多个线程间共享
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    println!("初始引用计数: {}", Arc::strong_count(&data));

    let mut handles = vec![];

    for i in 0..3 {
        // 克隆 Arc，增加引用计数
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 看到数据: {:?}", i, data);
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成，引用计数: {}", Arc::strong_count(&data));
}

// ============================================================================
// 示例 2: 共享可变状态 - Arc + Mutex
// ============================================================================
fn example2_arc_mutex() {
    // 使用 Arc 包装 Mutex，实现多线程间的共享可变状态
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("线程 {} 增加计数到 {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());
}

// ============================================================================
// 示例 3: Arc 与 Rc 的对比
// ============================================================================
use std::rc::Rc;
use std::time::Instant;

fn example3_rc_vs_arc() {
    // Rc 示例（单线程）
    let start = Instant::now();
    let rc_data = Rc::new(42);
    for _ in 0..1_000_000 {
        let _ = Rc::clone(&rc_data);
    }
    let rc_duration = start.elapsed();

    // Arc 示例（线程安全）
    let start = Instant::now();
    let arc_data = Arc::new(42);
    for _ in 0..1_000_000 {
        let _ = Arc::clone(&arc_data);
    }
    let arc_duration = start.elapsed();

    println!("Rc 克隆 1,000,000 次: {:?}", rc_duration);
    println!("Arc 克隆 1,000,000 次: {:?}", arc_duration);
    println!("性能差异: Arc 稍慢（原子操作开销）");
}

// ============================================================================
// 示例 4: 构建并发数据结构
// ============================================================================
#[derive(Debug)]
struct SharedList {
    data: Vec<i32>,
}

fn example4_concurrent_structure() {
    // 共享列表
    let list = Arc::new(Mutex::new(SharedList { data: vec![] }));
    let mut handles = vec![];

    // 多个线程向列表添加数据
    for i in 0..5 {
        let list = Arc::clone(&list);
        let handle = thread::spawn(move || {
            let mut list = list.lock().unwrap();
            list.data.push(i);
            println!("线程 {} 添加了 {}", i, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终列表: {:?}", list.lock().unwrap().data);
}

// ============================================================================
// 示例 5: Arc::downgrade 和 Weak
// ============================================================================

fn example5_arc_weak() {
    let strong = Arc::new(42);
    let weak = Arc::downgrade(&strong);

    println!("强引用计数: {}", Arc::strong_count(&strong));
    println!("弱引用计数: {}", Arc::weak_count(&strong));

    // 通过 Weak 尝试升级为 Arc
    if let Some(arc) = weak.upgrade() {
        println!("成功升级，值: {}", *arc);
    }

    drop(strong);

    // 强引用被 drop，Weak 无法升级
    if weak.upgrade().is_none() {
        println!("强引用已释放，Weak 无法升级");
    }
}

// ============================================================================
// 示例 6: 线程安全的引用计数数据
// ============================================================================
struct SharedData {
    value: i32,
}

fn example6_shared_data() {
    let data = Arc::new(SharedData { value: 100 });
    let mut handles = vec![];

    // 多个线程只读数据（安全）
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            println!("线程 {} 读取值: {}", i, data.value);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终引用计数: {}", Arc::strong_count(&data));
}

// ============================================================================
// 示例 7: Arc 的内存语义
// ============================================================================
fn example7_memory_semantics() {
    // Arc 提供顺序一致性内存顺序
    // 原子操作保证内存可见性
    let flag = Arc::new(std::sync::atomic::AtomicBool::new(false));
    let data = Arc::new(std::sync::atomic::AtomicI32::new(0));

    let flag_clone = Arc::clone(&flag);
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        // 写操作
        data_clone.store(42, std::sync::atomic::Ordering::SeqCst);
        flag_clone.store(true, std::sync::atomic::Ordering::SeqCst);
    });

    while !flag.load(std::sync::atomic::Ordering::SeqCst) {
        // 自旋等待
    }

    println!("读取到数据: {}", data.load(std::sync::atomic::Ordering::SeqCst));
    handle.join().unwrap();
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Arc<T> 原子引用计数智能指针示例 ===\n");

    println!("示例 1: 跨线程共享不可变数据");
    example1_basic_arc();
    println!();

    println!("示例 2: Arc + Mutex 共享可变状态");
    example2_arc_mutex();
    println!();

    println!("示例 3: Arc 与 Rc 性能对比");
    example3_rc_vs_arc();
    println!();

    println!("示例 4: 构建并发数据结构");
    example4_concurrent_structure();
    println!();

    println!("示例 5: Arc::downgrade 和 Weak");
    example5_arc_weak();
    println!();

    println!("示例 6: 线程安全的共享数据");
    example6_shared_data();
    println!();

    println!("示例 7: Arc 的内存语义");
    example7_memory_semantics();

    println!("\n=== 总结 ===");
    println!("Arc<T> 特点:");
    println!("  - Rc 的线程安全版本");
    println!("  - 使用原子操作，实现 Send + Sync");
    println!("  - 可以在多线程间安全共享");
    println!("  - 常与 Mutex/RwLock 组合使用");
    println!("  - 性能略低于 Rc（原子操作开销）");
    println!("  - 提供 Weak 引用，避免循环引用");
}
