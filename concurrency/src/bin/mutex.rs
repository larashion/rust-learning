// ============================================================================
// Mutex<T> - 互斥锁
// ============================================================================
//
// Mutex<T> 提供互斥访问，确保同一时间只有一个线程可以访问数据。
//
// 主要特点：
// 1. 互斥访问（Mutual Exclusion）
// 2. lock() 返回 MutexGuard（智能指针）
// 3. 自动释放锁（RAII）
// 4. 可能发生死锁
// 5. 通常与 Arc 配合使用

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// ============================================================================
// 示例 1: 基本 Mutex 用法
// ============================================================================
fn example1_basic_mutex() {
    // 创建 Mutex 包装的数据
    let m = Mutex::new(5);

    {
        // 获取锁，返回 MutexGuard
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("修改后: {}", *num);
    } // MutexGuard 在这里被 drop，锁自动释放

    // 可以再次获取锁
    println!("最终值: {}", *m.lock().unwrap());
}

// ============================================================================
// 示例 2: 多线程共享 Mutex
// ============================================================================
fn example2_shared_mutex() {
    // 使用 Arc 允许多个线程共享 Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", *counter.lock().unwrap());
}

// ============================================================================
// 示例 3: Mutex 与复杂类型
// ============================================================================
#[derive(Debug)]
struct SharedData {
    counter: i32,
    values: Vec<i32>,
}

fn example3_complex_type() {
    let data = Arc::new(Mutex::new(SharedData {
        counter: 0,
        values: vec![],
    }));

    let mut handles = vec![];

    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut data = data.lock().unwrap();
            data.counter += 1;
            data.values.push(i);
            println!("线程 {}: {:?}", i, *data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终数据: {:?}", *data.lock().unwrap());
}

// ============================================================================
// 示例 4: try_lock - 非阻塞获取锁
// ============================================================================
fn example4_try_lock() {
    // 互斥锁的所有权必须通过 Arc 共享
    let mutex = Arc::new(Mutex::new(0));
    let mutex1 = Arc::clone(&mutex);

    let handle = thread::spawn(move || {
        let _lock = mutex1.lock().unwrap();
        println!("线程1: 持有锁");
        thread::sleep(Duration::from_millis(500));
        println!("线程1: 释放锁");
    });

    thread::sleep(Duration::from_millis(100));

    println!("线程主: 尝试获取锁");
    match mutex.try_lock() {
        Ok(guard) => {
            println!("线程主: 获取锁成功");
            println!("线程主: 值 = {}", *guard);
        }
        Err(_) => {
            println!("线程主: 无法获取锁，已被占用");
        }
    }

    handle.join().unwrap();
}

// ============================================================================
// 示例 5: 死锁风险
// ============================================================================
fn example5_deadlock() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));

    let mutex1_a = Arc::clone(&mutex1);
    let mutex2_a = Arc::clone(&mutex2);
    let mutex1_b = Arc::clone(&mutex1);
    let mutex2_b = Arc::clone(&mutex2);

    let handle1 = thread::spawn(move || {
        println!("线程1: 获取锁1");
        let _lock1 = mutex1_a.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        println!("线程1: 尝试获取锁2（死锁！）");
        let _lock2 = mutex2_a.lock().unwrap();
        println!("线程1: 永远不会执行到这里");
    });

    let handle2 = thread::spawn(move || {
        println!("线程2: 获取锁2");
        let _lock2 = mutex2_b.lock().unwrap();
        thread::sleep(Duration::from_millis(100));
        println!("线程2: 尝试获取锁1（死锁！）");
        let _lock1 = mutex1_b.lock().unwrap();
        println!("线程2: 永远不会执行到这里");
    });

    // 这个例子会死锁！
    // 在实际代码中，应该总是以相同的顺序获取锁
    println!("警告: 这个例子会死锁，使用 Ctrl+C 终止");

    // 取消下面的注释来运行（会死锁）
    // handle1.join().unwrap();
    // handle2.join().unwrap();

    handle1.thread().unpark();
    handle2.thread().unpark();
}

// ============================================================================
// 示例 6: 正确的多锁获取（避免死锁）
// ============================================================================
fn example6_correct_lock_order() {
    let mutex1 = Arc::new(Mutex::new(0));
    let mutex2 = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for i in 0..2 {
        let mutex1 = Arc::clone(&mutex1);
        let mutex2 = Arc::clone(&mutex2);
        let handle = thread::spawn(move || {
            // 总是以相同的顺序获取锁
            let _lock1 = mutex1.lock().unwrap();
            let _lock2 = mutex2.lock().unwrap();
            println!("线程 {} 同时持有两个锁", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有线程完成，没有死锁");
}

// ============================================================================
// 示例 7: MutexGuard 实现 Deref 和 DerefMut
// ============================================================================
fn example7_mutex_guard() {
    let mutex = Mutex::new(String::from("hello"));

    // MutexGuard 实现了 Deref
    let guard = mutex.lock().unwrap();
    println!("长度: {}", guard.len()); // 可以直接调用 String 的方法

    // MutexGuard 实现了 DerefMut
    let mut guard = mutex.lock().unwrap();
    guard.push_str(" world"); // 可以修改
    println!("值: {}", *guard);
}

// ============================================================================
// 示例 8: 使用 Mutex 实现简单的线程安全计数器
// ============================================================================
struct ThreadSafeCounter {
    count: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    fn new() -> ThreadSafeCounter {
        ThreadSafeCounter {
            count: Arc::new(Mutex::new(0)),
        }
    }

    fn increment(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }
}

fn example8_thread_safe_counter() {
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = counter.count.clone();
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("计数器值: {}", counter.get());
}

// ============================================================================
// 示例 9: Mutex 泛型
// ============================================================================
fn example9_generic_mutex<T: Clone + Send + 'static>(initial: T, count: usize) -> Vec<T> {
    let data = Arc::new(Mutex::new(initial));
    let mut handles = vec![];

    for i in 0..count {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let _data = data.lock().unwrap();
            // 这里应该有修改操作
            println!("线程 {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let ret = (*data.lock().unwrap()).clone();
    vec![ret]
}

// ============================================================================
// 示例 10: 锁的毒化 (Poisoning)
// ============================================================================
fn example10_poisoning() {
    let mutex = Arc::new(Mutex::new(42));
    let mutex_clone = Arc::clone(&mutex);

    let handle = thread::spawn(move || {
        let mut data = mutex_clone.lock().unwrap();
        *data = 100;
        panic!("线程 panic！");
    });

    // 等待线程 panic
    let _ = handle.join();

    // Mutex 被"毒化"了
    let result = mutex.lock();
    match result {
        Ok(guard) => {
            println!("获取锁成功: {}", *guard);
        }
        Err(e) => {
            // 可以通过 into_inner() 恢复数据
            println!("锁已被毒化: {:?}", e);
            let recovered = e.into_inner();
            println!("恢复的值: {}", *recovered);
        }
    }
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Mutex<T> 互斥锁示例 ===\n");

    println!("示例 1: 基本 Mutex 用法");
    example1_basic_mutex();
    println!();

    println!("示例 2: 多线程共享 Mutex");
    example2_shared_mutex();
    println!();

    println!("示例 3: Mutex 与复杂类型");
    example3_complex_type();
    println!();

    println!("示例 4: try_lock - 非阻塞获取锁");
    example4_try_lock();
    println!();

    println!("示例 5: 死锁风险");
    example5_deadlock();
    println!();

    println!("示例 6: 正确的多锁获取（避免死锁）");
    example6_correct_lock_order();
    println!();

    println!("示例 7: MutexGuard 实现 Deref 和 DerefMut");
    example7_mutex_guard();
    println!();

    println!("示例 8: 使用 Mutex 实现简单的线程安全计数器");
    example8_thread_safe_counter();
    println!();

    println!("示例 9: Mutex 泛型");
    let _ = example9_generic_mutex("test", 3);
    println!();

    println!("示例 10: 锁的毒化 (Poisoning)");
    example10_poisoning();

    println!("\n=== 总结 ===");
    println!("Mutex<T> 特点:");
    println!("  - 提供互斥访问");
    println!("  - lock() 返回 MutexGuard（RAII）");
    println!("  - 自动释放锁（drop 时）");
    println!("  - 与 Arc 配合用于多线程共享");
    println!("  - 注意死锁风险");
    println!("  - try_lock() 非阻塞尝试");
    println!("  - Panic 会毒化 Mutex");
    println!("  - MutexGuard 实现 Deref/DerefMut");
}
