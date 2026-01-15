#![allow(unused)]
// ============================================================================ 
// RwLock<T> - 读写锁
// ============================================================================ 
//
// RwLock<T> 允许多个读者或一个写者，提供更灵活的并发控制。
//
// 主要特点：
// 1. 多读者单写者（Multiple Readers, Single Writer）
// 2. 读操作不互斥，写操作互斥
// 3. read() 返回 RwLockReadGuard
// 4. write() 返回 RwLockWriteGuard
// 5. 可能发生写者饥饿

use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;

// ============================================================================ 
// 示例 1: 基本读写操作
// ============================================================================ 
fn example1_basic_read_write() {
    let lock = RwLock::new(5);

    // 读操作
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap(); // 可以同时有多个读锁
        println!("读操作 1: {}", *r1);
        println!("读操作 2: {}", *r2);
    }

    // 写操作
    {
        let mut w = lock.write().unwrap();
        *w = 10;
        println!("写操作后: {}", *w);
    }

    // 再次读取
    {
        let r = lock.read().unwrap();
        println!("最终值: {}", *r);
    }
}

// ============================================================================ 
// 示例 2: 多个读者
// ============================================================================ 
fn example2_multiple_readers() {
    let lock = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // 创建 5 个读者线程
    for i in 0..5 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            let data = lock.read().unwrap();
            println!("读者 {:?}: {:?}", i, *data);
            thread::sleep(Duration::from_millis(100));
            println!("读者 {:?}: 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("所有读者完成");
}

// ============================================================================ 
// 示例 3: 读者与写者
// ============================================================================ 
fn example3_readers_and_writers() {
    let lock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 创建读者
    for i in 0..3 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _ in 0..3 {
                let data = lock.read().unwrap();
                println!("读者 {:?}: 读取 {}", i, *data);
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    // 创建写者
    for i in 0..2 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _j in 0..2 {
                let mut data = lock.write().unwrap();
                *data += 10;
                println!("写者 {:?}: 写入 {}", i, *data);
                thread::sleep(Duration::from_millis(100));
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", *lock.read().unwrap());
}

// ============================================================================ 
// 示例 4: try_read 和 try_write
// ============================================================================ 
fn example4_try_methods() {
    let lock = Arc::new(RwLock::new(42));
    let lock_clone = Arc::clone(&lock);

    // 持有写锁
    let _write_guard = lock.write().unwrap();

    // 尝试读锁（会失败）
    let result = lock_clone.try_read();
    match result {
        Ok(_guard) => {
            println!("获取读锁成功");
        }
        Err(_) => {
            println!("无法获取读锁（写锁被持有）");
        }
    }

    // 尝试写锁（会失败）
    let result = lock_clone.try_write();
    match result {
        Ok(_guard) => {
            println!("获取写锁成功");
        }
        Err(_) => {
            println!("无法获取写锁（写锁已被持有）");
        }
    }
}

// ============================================================================ 
// 示例 5: RwLock 与复杂类型
// ============================================================================ 
#[derive(Debug)]
struct Database {
    data: Vec<String>,
}

fn example5_complex_type() {
    let db = Arc::new(RwLock::new(Database { data: vec![] }));
    let mut handles = vec![];

    // 写者添加数据
    for i in 0..3 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db = db.write().unwrap();
            db.data.push(format!("数据 {}", i));
            println!("写者: 添加数据 {}", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 重新初始化 handles，因为之前的 vector 已经被移动
    let mut handles = vec![];

    // 读者读取数据
    for i in 0..3 {
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let db = db.read().unwrap();
            println!("读者 {:?}: {:?}", i, db.data);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

// ============================================================================ 
// 示例 6: 读写锁的性能优势
// ============================================================================ 
fn example6_performance_comparison() {
    use std::sync::Mutex;

    let rwlock = Arc::new(RwLock::new(0));
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    let start = std::time::Instant::now();

    // 使用 RwLock 进行读操作
    for i in 0..10 {
        let lock = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let _guard = lock.read().unwrap();
            }
            println!("读者 {} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let rwlock_duration = start.elapsed();
    let mut handles = vec![];

    let start = std::time::Instant::now();

    // 使用 Mutex 进行读操作
    for i in 0..10 {
        let lock = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let _guard = lock.lock().unwrap();
            }
            println!("读者 {} 完成", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mutex_duration = start.elapsed();

    println!("RwLock 读操作耗时: {:?}", rwlock_duration);
    println!("Mutex 读操作耗时: {:?}", mutex_duration);
    println!("注: 多读者场景下 RwLock 性能更好");
}

// ============================================================================ 
// 示例 7: 升级和降级锁（通过重新获取）
// ============================================================================ 
fn example7_lock_downgrade() {
    let lock = Arc::new(RwLock::new(42));
    let lock_clone = Arc::clone(&lock);

    // 持有写锁
    {
        let mut write_guard = lock.write().unwrap();
        *write_guard = 100;
        println!("写锁: 修改为 {}", *write_guard);

        // 写锁结束后可以获取读锁
        // 注意：RwLock 不支持直接降级，需要先 drop 写锁
    }

    // 获取读锁
    {
        let read_guard = lock_clone.read().unwrap();
        println!("读锁: 读取 {}", *read_guard);
    }
}

// ============================================================================ 
// 示例 8: 写者饥饿问题
// ============================================================================ 
fn example8_writer_starvation() {
    let lock = Arc::new(RwLock::new(0));
    let mut handles = vec![];

    // 持续不断的读者
    for i in 0..5 {
        let lock = Arc::clone(&lock);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let _guard = lock.read().unwrap();
                thread::sleep(Duration::from_millis(1));
            }
            println!("读者 {} 完成", i);
        });
        handles.push(handle);
    }

    // 尝试获取写锁
    let lock_clone = Arc::clone(&lock);
    let handle = thread::spawn(move || {
        println!("写者: 尝试获取写锁...");
        let mut guard = lock_clone.write().unwrap();
        println!("写者: 获取写锁成功！");
        *guard = 999;
        thread::sleep(Duration::from_millis(100));
        println!("写者: 完成");
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终值: {}", *lock.read().unwrap());
}

// ============================================================================ 
// 示例 9: RwLock 的毒化
// ============================================================================ 
fn example9_poisoning() {
    let lock = Arc::new(RwLock::new(42));
    let lock_clone = Arc::clone(&lock);

    let handle = thread::spawn(move || {
        let mut data = lock_clone.write().unwrap();
        *data = 100;
        panic!("线程 panic！");
    });

    // 等待线程 panic
    let _ = handle.join();

    // RwLock 也可能被毒化
    let result = lock.read();
    match result {
        Ok(guard) => {
            println!("读锁获取成功: {}", *guard);
        }
        Err(e) => {
            println!("读锁已被毒化: {:?}", e);
            let recovered = e.into_inner();
            println!("恢复的值: {}", *recovered);
        }
    }
}

// ============================================================================ 
// 示例 10: 实际应用场景 - 缓存系统
// ============================================================================ 
struct Cache {
    data: RwLock<Vec<(String, String)>>,
}

impl Cache {
    fn new() -> Cache {
        Cache {
            data: RwLock::new(vec![]),
        }
    }

    fn get(&self, key: &str) -> Option<String> {
        let data = self.data.read().unwrap();
        data.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone())
    }

    fn set(&self, key: String, value: String) {
        let mut data = self.data.write().unwrap();
        data.push((key, value));
    }

    fn list(&self) -> Vec<(String, String)> {
        let data = self.data.read().unwrap();
        data.clone()
    }
}

fn example10_cache_system() {
    let cache = Arc::new(Cache::new());
    let mut handles = vec![];

    // 多个写者
    for i in 0..3 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let key = format!("key_{}_{}", i, j);
                let value = format!("value_{}_{}", i, j);
                cache.set(key, value);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // 重新初始化 handles
    let mut handles = vec![];

    // 多个读者
    for i in 0..5 {
        let cache = Arc::clone(&cache);
        let handle = thread::spawn(move || {
            let items = cache.list();
            println!("读者 {}: {} 个条目", i, items.len());
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("缓存内容: {:?}", cache.list());
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== RwLock<T> 读写锁示例 ===\n");

    println!("示例 1: 基本读写操作");
    example1_basic_read_write();
    println!();

    println!("示例 2: 多个读者");
    example2_multiple_readers();
    println!();

    println!("示例 3: 读者与写者");
    example3_readers_and_writers();
    println!();

    println!("示例 4: try_read 和 try_write");
    example4_try_methods();
    println!();

    println!("示例 5: RwLock 与复杂类型");
    example5_complex_type();
    println!();

    println!("示例 6: 读写锁的性能优势");
    example6_performance_comparison();
    println!();

    println!("示例 7: 升级和降级锁");
    example7_lock_downgrade();
    println!();

    println!("示例 8: 写者饥饿问题");
    example8_writer_starvation();
    println!();

    println!("示例 9: RwLock 的毒化");
    example9_poisoning();
    println!();

    println!("示例 10: 实际应用场景 - 缓存系统");
    example10_cache_system();

    println!("\n=== 总结 ===");
    println!("RwLock<T> 特点:");
    println!("  - 多读者单写者（MRSW）");
    println!("  - 读操作不互斥，写操作互斥");
    println!("  - read() 返回 RwLockReadGuard");
    println!("  - write() 返回 RwLockWriteGuard");
    println!("  - try_read/try_write 非阻塞");
    println!("  - 适合读多写少的场景");
    println!("  - 注意写者饥饿问题");
    println!("  - 也会被毒化");
}