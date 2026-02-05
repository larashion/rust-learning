use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn atomic_counter(thread_number: usize, increments: usize) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..thread_number {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = counter.load(Ordering::SeqCst);
    println!("最终计数: {}", final_count);
    final_count
}

fn main() {
    println!("=== 示例: 多线程原子计数器 ===");
    atomic_counter(20, 1000);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_atomic_counter_basic() {
        assert_eq!(atomic_counter(10, 100), 1000);
    }

    #[test]
    fn test_atomic_counter_zero_threads() {
        assert_eq!(atomic_counter(0, 100), 0);
    }

    #[test]
    fn test_atomic_counter_zero_increments() {
        assert_eq!(atomic_counter(10, 0), 0);
    }

    #[test]
    fn test_atomic_counter_large() {
        // Ensure it handles reasonably large numbers without overflow (in this context)
        assert_eq!(atomic_counter(5, 2000), 10000);
    }
}
