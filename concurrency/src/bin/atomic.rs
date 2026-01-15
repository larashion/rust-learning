#![allow(unused)]
// ============================================================================ 
// 原子类型 (Atomic Types) - 无锁并发
// ============================================================================ 
// 
// Rust 的原子类型提供无锁的并发操作，通过硬件支持的原子指令实现。
// 
// 主要特点：
// 1. 无锁并发（lock-free）
// 2. 使用 CPU 原子指令
// 3. 支持不同的内存顺序（Memory Ordering）
// 4. 适合实现简单的计数器、标志等
// 5. 性能通常优于 Mutex

use std::sync::atomic::{AtomicBool, AtomicI32, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

// ============================================================================ 
// 示例 1: 基本的原子操作
// ============================================================================ 
fn example1_basic_atomic() {
    let atomic = Arc::new(AtomicI32::new(0));

    // 读取
    println!("初始值: {}", atomic.load(Ordering::SeqCst));

    // 写入
    atomic.store(10, Ordering::SeqCst);
    println!("写入后: {}", atomic.load(Ordering::SeqCst));

    // 原子地加值
    atomic.fetch_add(5, Ordering::SeqCst);
    println!("加 5 后: {}", atomic.load(Ordering::SeqCst));

    // 原子地减值
    atomic.fetch_sub(3, Ordering::SeqCst);
    println!("减 3 后: {}", atomic.load(Ordering::SeqCst));
}

// ============================================================================ 
// 示例 2: 多线程原子计数器
// ============================================================================ 
fn example2_atomic_counter() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", counter.load(Ordering::SeqCst));
}

// ============================================================================ 
// 示例 3: 内存顺序 (Memory Ordering)
// ============================================================================ 
fn example3_memory_ordering() {
    let data = Arc::new(AtomicI32::new(0));

    // Relaxed: 最宽松，只保证原子性
    data.fetch_add(1, Ordering::Relaxed);
    println!("Relaxed: {}", data.load(Ordering::Relaxed));

    // Acquire: 保证读操作不会重排序
    let value = data.load(Ordering::Acquire);
    println!("Acquire: {}", value);

    // Release: 保证写操作不会重排序
    data.store(10, Ordering::Release);
    println!("Release: {}", data.load(Ordering::Relaxed));

    // SeqCst: 顺序一致性，最强保证
    data.fetch_add(1, Ordering::SeqCst);
    println!("SeqCst: {}", data.load(Ordering::SeqCst));
}

// ============================================================================ 
// 示例 4: 比较并交换 (Compare and Swap)
// ============================================================================ 
fn example4_compare_and_swap() {
    let atomic = Arc::new(AtomicI32::new(5));

    // 比较并交换：如果当前值是 5，则改为 10
    let old_value = atomic.compare_exchange(
        5,      // 期望的当前值
        10,     // 新值
        Ordering::SeqCst,
        Ordering::SeqCst
    );

    match old_value {
        Ok(v) => println!("CAS 成功: {} -> {}", v, atomic.load(Ordering::SeqCst)),
        Err(v) => println!("CAS 失败: 当前值是 {}", v),
    }

    // 再次尝试（会失败，因为值已经是 10）
    let old_value = atomic.compare_exchange(
        5,
        20,
        Ordering::SeqCst,
        Ordering::SeqCst
    );

    match old_value {
        Ok(v) => println!("CAS 成功: {} -> {}", v, atomic.load(Ordering::SeqCst)),
        Err(v) => println!("CAS 失败: 当前值是 {}", v),
    }
}

// ============================================================================ 
// 示例 5: 原子布尔值
// ============================================================================ 
fn example5_atomic_bool() {
    let flag = Arc::new(AtomicBool::new(false));
    let flag_clone = Arc::clone(&flag);

    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        flag_clone.store(true, Ordering::SeqCst);
        println!("线程: 设置标志为 true");
    });

    // 自旋等待
    while !flag.load(Ordering::SeqCst) {
        // 自旋等待
    }

    println!("主线程: 检测到标志为 true");
    handle.join().unwrap();
}

// ============================================================================ 
// 示例 6: 原子自增和自减
// ============================================================================ 
fn example6_fetch_add_sub() {
    let counter = Arc::new(AtomicI32::new(0));

    // 自增，返回旧值
    let old = counter.fetch_add(1, Ordering::SeqCst);
    println!("fetch_add 返回旧值: {}", old);
    println!("当前值: {}", counter.load(Ordering::SeqCst));

    // 自增，返回新值
    let new = counter.fetch_add(1, Ordering::SeqCst).wrapping_add(1);
    println!("fetch_add + 1 返回新值: {}", new);
    println!("当前值: {}", counter.load(Ordering::SeqCst));

    // 自减
    counter.fetch_sub(1, Ordering::SeqCst);
    println!("fetch_sub 后: {}", counter.load(Ordering::SeqCst));
}

// ============================================================================ 
// 示例 7: 原子位操作
// ============================================================================ 
fn example7_bit_operations() {
    let atomic = Arc::new(AtomicI32::new(0b0000));

    // 按位或
    atomic.fetch_or(0b1010, Ordering::SeqCst);
    println!("fetch_or: {:04b}", atomic.load(Ordering::SeqCst));

    // 按位与
    atomic.fetch_and(0b1100, Ordering::SeqCst);
    println!("fetch_and: {:04b}", atomic.load(Ordering::SeqCst));

    // 按位异或
    atomic.fetch_xor(0b1111, Ordering::SeqCst);
    println!("fetch_xor: {:04b}", atomic.load(Ordering::SeqCst));
}

// ============================================================================ 
// 示例 8: 原子最大值和最小值
// ============================================================================ 
fn example8_max_min() {
    let atomic = Arc::new(AtomicI32::new(5));

    // 原子地取最大值
    atomic.fetch_max(10, Ordering::SeqCst);
    println!("fetch_max(10): {}", atomic.load(Ordering::SeqCst));

    // 原子地取最小值
    atomic.fetch_min(3, Ordering::SeqCst);
    println!("fetch_min(3): {}", atomic.load(Ordering::SeqCst));
}

// ============================================================================ 
// 示例 9: 自旋锁实现（使用原子类型）
// ============================================================================ 
struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    fn new() -> SpinLock {
        SpinLock {
            locked: AtomicBool::new(false),
        }
    }

    fn lock(&self) {
        // 自旋等待，直到获取锁
        while self.locked.compare_exchange(
            false,
            true,
            Ordering::Acquire,
            Ordering::Relaxed
        ).is_err() {
            // 自旋
            std::hint::spin_loop();
        }
    }

    fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

fn example9_spinlock() {
    let lock = Arc::new(SpinLock::new());
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let lock = Arc::clone(&lock);
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                lock.lock();
                counter.fetch_add(1, Ordering::Relaxed);
                lock.unlock();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("自旋锁保护的计数: {}", counter.load(Ordering::Relaxed));
}

// ============================================================================ 
// 示例 10: 生产者-消费者模式（原子版本）
// ============================================================================ 
fn example10_atomic_producer_consumer() {
    // 简化的版本，实际应用中通常使用 channel
    let data = Arc::new(AtomicI32::new(0));
    let ready = Arc::new(AtomicBool::new(false));
    let consumed = Arc::new(AtomicBool::new(false));

    let data_clone = Arc::clone(&data);
    let ready_clone = Arc::clone(&ready);
    let consumed_clone = Arc::clone(&consumed);

    // 生产者
    let producer = thread::spawn(move || {
        for i in 1..=5 {
            // 等待消费完成
            while consumed_clone.load(Ordering::Acquire) {
                thread::yield_now();
            }

            // 生产数据
            data_clone.store(i, Ordering::Release);
            consumed_clone.store(false, Ordering::Release);
            ready_clone.store(true, Ordering::Release);
            println!("生产者: 生产 {}", i);
        }
    });

    // 消费者
    let consumer = thread::spawn(move || {
        for _ in 1..=5 {
            // 等待数据就绪
            while !ready.load(Ordering::Acquire) {
                thread::yield_now();
            }

            // 消费数据
            let value = data.load(Ordering::Acquire);
            ready.store(false, Ordering::Release);
            consumed.store(true, Ordering::Release);
            println!("消费者: 消费 {}", value);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

// ============================================================================ 
// 示例 11: 使用原子类型实现引用计数
// ============================================================================ 
struct ArcLike<T> {
    data: T,
    ref_count: Arc<AtomicUsize>,
}

impl<T> ArcLike<T> {
    fn new(data: T) -> ArcLike<T> {
        ArcLike {
            data,
            ref_count: Arc::new(AtomicUsize::new(1)),
        }
    }

    fn clone(&self) -> ArcLike<T> {
        self.ref_count.fetch_add(1, Ordering::Relaxed);
        ArcLike {
            data: unsafe { std::ptr::read(&self.data as *const _) },
            ref_count: Arc::clone(&self.ref_count),
        }
    }
}

impl<T> Drop for ArcLike<T> {
    fn drop(&mut self) {
        if self.ref_count.fetch_sub(1, Ordering::Release) == 1 {
            unsafe { std::ptr::drop_in_place(&mut self.data) };
        }
    }
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== 原子类型 (Atomic Types) 示例 ===\n");

    println!("示例 1: 基本的原子操作");
    example1_basic_atomic();
    println!();

    println!("示例 2: 多线程原子计数器");
    example2_atomic_counter();
    println!();

    println!("示例 3: 内存顺序 (Memory Ordering)");
    example3_memory_ordering();
    println!();

    println!("示例 4: 比较并交换 (Compare and Swap)");
    example4_compare_and_swap();
    println!();

    println!("示例 5: 原子布尔值");
    example5_atomic_bool();
    println!();

    println!("示例 6: 原子自增和自减");
    example6_fetch_add_sub();
    println!();

    println!("示例 7: 原子位操作");
    example7_bit_operations();
    println!();

    println!("示例 8: 原子最大值和最小值");
    example8_max_min();
    println!();

    println!("示例 9: 自旋锁实现");
    example9_spinlock();
    println!();

    println!("示例 10: 生产者-消费者模式（原子版本）");
    example10_atomic_producer_consumer();

    println!("\n=== 总结 ===");
    println!("原子类型特点:");
    println!("  - 无锁并发（lock-free）");
    println!("  - 使用 CPU 原子指令");
    println!("  - 支持不同内存顺序");
    println!("  - 适合简单计数器、标志");
    println!("  - 性能优于 Mutex");
    println!("  - 可用类型: AtomicI8, AtomicI16, AtomicI32, AtomicI64,");
    println!("              AtomicU8, AtomicU16, AtomicU32, AtomicU64,");
    println!("              AtomicIsize, AtomicUsize, AtomicBool");
    println!("  - 操作: load, store, fetch_add, fetch_sub,");
    println!("         fetch_or, fetch_and, fetch_xor,");
    println!("         compare_exchange, swap");
}