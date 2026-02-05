use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn example_atomic_counter() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("最终计数: {}", counter.load(Ordering::Relaxed));
}

fn main() {
    println!("=== 示例: 多线程原子计数器 ===");
    example_atomic_counter();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_increment() {
        let counter = AtomicUsize::new(0);
        counter.fetch_add(1, Ordering::Relaxed);
        counter.fetch_add(5, Ordering::Relaxed);
        assert_eq!(counter.load(Ordering::Relaxed), 6);
    }

    #[test]
    fn test_multithreaded_counter() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];
        for _ in 0..20 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                for _ in 0..100 {
                    c.fetch_add(1, Ordering::Relaxed);
                }
            }));
        }
        for h in handles {
            h.join().unwrap();
        }
        assert_eq!(counter.load(Ordering::Relaxed), 2000);
    }
}
