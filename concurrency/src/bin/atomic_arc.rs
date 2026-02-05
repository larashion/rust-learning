use std::ops::Deref;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicUsize, Ordering};

struct ArcInner<T> {
    rc: AtomicUsize,
    data: T,
}

pub struct MyArc<T> {
    ptr: NonNull<ArcInner<T>>,
}

unsafe impl<T: Send + Sync> Sync for MyArc<T> {}
unsafe impl<T: Send + Sync> Send for MyArc<T> {}

impl<T> MyArc<T> {
    pub fn new(data: T) -> Self {
        let inner = Box::new(ArcInner {
            rc: AtomicUsize::new(1),
            data,
        });
        Self {
            ptr: NonNull::new(Box::into_raw(inner)).unwrap(),
        }
    }
}

impl<T> Clone for MyArc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.ptr.as_ref() };
        // 使用 Relaxed 是因为我们不需要同步这里之前或之后的操作，
        // 只要保证计数器本身增加是原子性的即可。
        inner.rc.fetch_add(1, Ordering::Relaxed);
        MyArc { ptr: self.ptr }
    }
}

impl<T> Deref for MyArc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        let inner = unsafe { self.ptr.as_ref() };
        &inner.data
    }
}

impl<T> Drop for MyArc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.ptr.as_ref() };
        // 使用 Release 确保当前线程对数据的访问/修改对其他线程可见
        if inner.rc.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        // 当计数降到 0 时，我们需要 Acquire 屏障，以确保我们能看到
        // 之前所有其他线程对数据的操作（与上面的 Release 配对）
        std::sync::atomic::fence(Ordering::Acquire);

        // SAFETY: 引用计数为 0，且我们有独占访问权（通过 fence 保证），
        // 销毁代码只执行一次。
        unsafe {
            drop(Box::from_raw(self.ptr.as_ptr()));
        }
    }
}
fn example_myarc() {
    let s1 = MyArc::new(String::from("Hello World"));
    let s2 = s1.clone();

    println!("s1 地址: {:p}", &*s1);
    println!("s2 地址: {:p}", &*s2);
    println!("内容: {:?}", *s2);
}

fn main() {
    println!("=== 示例: 实现原子引用计数 ===");
    example_myarc();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use std::thread;

    #[test]
    fn test_arc_basic() {
        let x = MyArc::new(5);
        let y = x.clone();
        assert_eq!(*x, 5);
        assert_eq!(*y, 5);
    }

    #[test]
    fn test_arc_drop_behavior() {
        static DROPPED: AtomicBool = AtomicBool::new(false);

        struct Trap;
        impl Drop for Trap {
            fn drop(&mut self) {
                DROPPED.store(true, Ordering::Release);
            }
        }

        {
            let arc1 = MyArc::new(Trap);
            let _arc2 = arc1.clone();
            assert!(!DROPPED.load(Ordering::Acquire));
        } // 两个 Arc 都应该在这里被 drop

        assert!(DROPPED.load(Ordering::Acquire));
    }

    #[test]
    fn test_arc_multithreaded() {
        let val = MyArc::new(100);
        let mut handles = vec![];

        for _ in 0..10 {
            let v = val.clone();
            handles.push(thread::spawn(move || {
                assert_eq!(*v, 100);
            }));
        }
        handles.into_iter().for_each(|h| h.join().unwrap());
    }
}
