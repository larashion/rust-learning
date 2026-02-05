use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;

pub struct SpinLockGuard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<'a, T> Drop for SpinLockGuard<'a, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: 我们持有锁，且锁提供了互斥保证，
        // 所以可以安全地分发可变引用。
        unsafe { &mut *self.lock.data.get() }
    }
}

struct SpinLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

// SAFETY: 只要 T 是 Send 的，SpinLock<T> 就可以在线程间安全传递（Sync）。
// 因为 lock() 机制保证了同一时间只有一个线程能访问内部数据。
unsafe impl<T: Send> Sync for SpinLock<T> {}

impl<T> SpinLock<T> {
    fn new(data: T) -> SpinLock<T> {
        SpinLock {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    fn lock(&self) -> SpinLockGuard<'_, T> {
        loop {
            // 先进行简单的 load 检查，减少对缓存行的独占争用
            while self.locked.load(Ordering::Relaxed) {
                // 通知 CPU 我在自旋
                std::hint::spin_loop();
            }
            // 尝试获取锁：Acquire 确保我们在拿到锁之后，才能看到受保护数据的变化
            if self
                .locked
                .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                return SpinLockGuard { lock: self };
            }
        }
    }
}

fn example_spinlock(thread_number: usize, ops_number: usize) -> usize {
    let counter = Arc::new(SpinLock::new(0));
    let mut handles = vec![];

    for _ in 0..thread_number {
        let spinlock = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..ops_number {
                *spinlock.lock() += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    let res = *counter.lock();
    res
}

fn main() {
    println!("=== 示例: 自旋锁实现 (SpinLock) ===");
    println!("自旋锁保护的计数结果: {}", example_spinlock(200, 50));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spinlock() {
        assert_eq!(example_spinlock(10, 100), 1000);
        assert_eq!(example_spinlock(0, 100), 0);
    }
}
