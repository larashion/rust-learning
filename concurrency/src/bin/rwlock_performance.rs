use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

/// 通用的性能测试函数，负责线程调度和计时
fn run_benchmark<F>(label: &str, num_threads: usize, iters: usize, task: F) -> Duration
where
    F: Fn() + Send + Sync + 'static + Clone,
{
    let start = Instant::now();
    let mut handles = vec![];
    for i in 0..num_threads {
        let task = task.clone();
        let label = label.to_string();
        let handle = thread::spawn(move || {
            (0..iters).for_each(|_| task());
            println!("{} 写者 {} 完成", label, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    start.elapsed()
}

/// 比较 RwLock 和 Mutex 在高频率读操作场景下的性能差异
fn compare_rwlock_vs_mutex_performance() {
    let rwlock = Arc::new(RwLock::new(0));
    let mutex = Arc::new(Mutex::new(0));
    let threads = 10;
    let iterations = 1000;

    // 1. RwLock 性能测试
    let rwlock_duration = run_benchmark("RwLock", threads, iterations, {
        let lock = Arc::clone(&rwlock);
        move || {
            let _guard = lock.read().unwrap();
        }
    });

    // 2. Mutex 性能测试
    let mutex_duration = run_benchmark("Mutex", threads, iterations, {
        let lock = Arc::clone(&mutex);
        move || {
            let _guard = lock.lock().unwrap();
        }
    });

    println!("\n结果对比:");
    println!("RwLock 读操作总耗时: {:?}", rwlock_duration);
    println!("Mutex 读操作总耗时: {:?}", mutex_duration);
    println!("结论: 在多读者场景下，RwLock 由于允许并发读取，性能通常优于 Mutex。");
}

fn main() {
    println!("=== RwLock<T> 性能对比示例 ===");
    compare_rwlock_vs_mutex_performance();
}
