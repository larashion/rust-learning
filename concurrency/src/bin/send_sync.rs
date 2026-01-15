// ============================================================================
// Send 和 Sync trait - 线程安全标记
// ============================================================================
//
// Send 和 Sync 是 Rust 中用于标记线程安全性的 marker traits。
//
// 主要特点：
// 1. Send: 类型可以安全地在线程间转移所有权
// 2. Sync: 类型的引用可以安全地在线程间共享
// 3. 这两个 trait 都是自动推导的（通常不需要手动实现）
// 4. 违反安全性规则会有编译错误
// 5. unsafe impl 需要确保安全性不变式

use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// ============================================================================
// 示例 1: Send trait 基本概念
// ============================================================================
fn example1_send_trait() {
    // Send: 类型可以跨线程转移所有权
    let data = String::from("hello");

    // String 是 Send，可以移动到新线程
    let handle = thread::spawn(move || {
        println!("线程接收到: {}", data);
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 2: 非 Send 类型 (Rc)
// ============================================================================
fn example2_non_send() {
    // Rc 不是 Send，因为它使用非原子引用计数
    let _data = Rc::new(42);

    // 以下代码会编译错误
    // let handle = thread::spawn(move || {
    //     println!("{}", data);
    // });
    // handle.join().unwrap();

    println!("Rc 不是 Send，不能在线程间转移");
    println!("解决方案: 使用 Arc（原子引用计数）");

    let arc_data = Arc::new(42);
    let handle = thread::spawn(move || {
        println!("Arc 是 Send: {}", arc_data);
    });
    handle.join().unwrap();
}

// ============================================================================
// 示例 3: Sync trait 基本概念
// ============================================================================
fn example3_sync_trait() {
    // Sync: 类型的引用可以跨线程共享
    let data = Arc::new(42);

    // Arc<i32> 是 Sync，可以在线程间共享引用
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let handle1 = thread::spawn(move || {
        println!("线程1: {}", data1);
    });

    let handle2 = thread::spawn(move || {
        println!("线程2: {}", data2);
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// ============================================================================
// 示例 4: 手动实现 Send (unsafe)
// ============================================================================
struct SafeSendData {
    data: i32,
}

// 安全条件: SafeSendData 不包含任何非 Send 的内部类型
unsafe impl Send for SafeSendData {}

fn example4_manual_send() {
    let data = SafeSendData { data: 42 };

    let handle = thread::spawn(move || {
        println!("手动实现的 Send: {}", data.data);
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 5: 手动实现 Sync (unsafe)
// ============================================================================
struct SafeSyncData {
    data: Mutex<i32>,
}

// 安全条件: &SafeSyncData 可以安全地跨线程共享
// 因为 Mutex<T> 是 Sync，当 T: Send 时
unsafe impl Sync for SafeSyncData {}

fn example5_manual_sync() {
    let data = Arc::new(SafeSyncData {
        data: Mutex::new(0),
    });

    let mut handles = vec![];

    for _ in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut num = data.data.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("计数: {}", *data.data.lock().unwrap());
}

// ============================================================================
// 示例 6: PhantomData 用于实现 Send/Sync
// ============================================================================
struct Wrapper<T> {
    data: T,
    _marker: PhantomData<*const ()>, // *const () 不是 Send/Sync
}

// 如果我们想要让 Wrapper 只有在某些条件下才是 Send/Sync
// 我们可以使用条件实现
unsafe impl<T: Send> Send for Wrapper<T> {}

fn example6_phantomdata() {
    let data = Wrapper {
        data: 42,
        _marker: PhantomData,
    };

    let handle = thread::spawn(move || {
        println!("Wrapper with i32: {}", data.data);
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 7: 类型自动推导规则
// ============================================================================
struct MyStruct {
    x: i32,
    y: String,
}

fn example7_auto_derive() {
    // MyStruct 自动实现 Send 和 Sync
    // 因为它的所有字段都是 Send 和 Sync

    let data = MyStruct {
        x: 10,
        y: String::from("hello"),
    };

    let handle = thread::spawn(move || {
        println!("MyStruct 是 Send: {}, {}", data.x, data.y);
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 8: 常见类型的 Send/Sync 状态
// ============================================================================
fn example8_type_status() {
    println!("常见类型的 Send/Sync 状态:");
    println!("  i32: Send: 是, Sync: 是");
    println!("  String: Send: 是, Sync: 是");
    println!("  Vec<T>: Send: 如果 T:Send, Sync: 如果 T:Sync");
    println!("  Box<T>: Send: 如果 T:Send, Sync: 如果 T:Sync");
    println!("  Rc<T>: Send: 否, Sync: 否");
    println!("  Arc<T>: Send: 如果 T:Send, Sync: 如果 T:Send+Sync");
    println!("  Mutex<T>: Send: 如果 T:Send, Sync: 如果 T:Send");
    println!("  *mut T: Send: 否, Sync: 否");
    println!("  unsafe fn(): Send: 否, Sync: 否");
}

// ============================================================================
// 示例 9: 组合智能指针的线程安全性
// ============================================================================
fn example9_smart_pointers_thread_safety() {
    // Arc<Mutex<T>>: Send if T:Send, Sync if T:Send
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

    println!("Arc<Mutex<i32>> 计数: {}", *counter.lock().unwrap());
}

// ============================================================================
// 示例 10: 函数指针的线程安全性
// ============================================================================
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn example10_function_pointer() {
    // 函数指针是 Send 如果它捕获的数据是 Send
    let func = is_even;

    let handle = thread::spawn(move || {
        println!("10 是偶数吗? {}", func(10));
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 11: 闭包的线程安全性
// ============================================================================
fn example11_closure_thread_safety() {
    let data = [1, 2, 3, 4, 5];

    // 闭包捕获数据，move 转移所有权
    // 因为 Vec<i32> 是 Send，闭包也是 Send
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("闭包计算和: {}", sum);
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 12: 原子类型的 Send/Sync
// ============================================================================
use std::sync::atomic::AtomicI32;

fn example12_atomic_types() {
    // 原子类型都是 Send 和 Sync
    let atomic = Arc::new(AtomicI32::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let atomic = Arc::clone(&atomic);
        let handle = thread::spawn(move || {
            atomic.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("AtomicI32 是 Send 和 Sync: {}",
             atomic.load(std::sync::atomic::Ordering::SeqCst));
}

// ============================================================================
// 示例 13: 自定义类型的条件 Send/Sync 实现
// ============================================================================
struct ConditionalSend<T> {
    data: T,
}

// 只有当 T: Send 时，ConditionalSend<T> 才是 Send
unsafe impl<T: Send> Send for ConditionalSend<T> {}

// 只有当 T: Sync 时，ConditionalSend<T> 才是 Sync
unsafe impl<T: Sync> Sync for ConditionalSend<T> {}

fn example13_conditional_send_sync() {
    // i32 是 Send，所以 ConditionalSend<i32> 是 Send
    let data = ConditionalSend { data: 42 };

    let handle = thread::spawn(move || {
        println!("ConditionalSend<i32>: {}", data.data);
    });

    handle.join().unwrap();

    // Arc<ConditionalSend<i32>> 是 Sync
    let data = Arc::new(ConditionalSend { data: 42 });
    let data_clone = Arc::clone(&data);

    let handle = thread::spawn(move || {
        println!("Arc<ConditionalSend<i32>>: {}", data_clone.data);
    });

    handle.join().unwrap();
}

// ============================================================================
// 示例 14: 检查类型是否实现 Send/Sync
// ============================================================================
fn example14_check_send_sync() {
    // 使用 trait bound 检查
    fn is_send<T: Send>(_value: &T) {
        println!("类型实现了 Send");
    }

    fn is_sync<T: Sync>(_value: &T) {
        println!("类型实现了 Sync");
    }

    let data = 42;
    is_send(&data);
    is_sync(&data);
}

// ============================================================================
// 示例 15: 使用 where 子句约束线程安全性
// ============================================================================
fn example15_where_clause() {
    fn spawn_with_check<T: Send + 'static>(_data: T) {
        let handle = thread::spawn(move || {
            println!("接收数据");
            // data 在这里可以使用
        });
        handle.join().unwrap();
    }

    let data = String::from("hello");
    spawn_with_check(data);

    let data = vec
![1, 2, 3];
    spawn_with_check(data);
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== Send 和 Sync trait 示例 ===\n");

    println!("示例 1: Send trait 基本概念");
    example1_send_trait();
    println!();

    println!("示例 2: 非 Send 类型 (Rc)");
    example2_non_send();
    println!();

    println!("示例 3: Sync trait 基本概念");
    example3_sync_trait();
    println!();

    println!("示例 4: 手动实现 Send (unsafe)");
    example4_manual_send();
    println!();

    println!("示例 5: 手动实现 Sync (unsafe)");
    example5_manual_sync();
    println!();

    println!("示例 6: PhantomData 用于实现 Send/Sync");
    example6_phantomdata();
    println!();

    println!("示例 7: 类型自动推导规则");
    example7_auto_derive();
    println!();

    println!("示例 8: 常见类型的 Send/Sync 状态");
    example8_type_status();
    println!();

    println!("示例 9: 组合智能指针的线程安全性");
    example9_smart_pointers_thread_safety();
    println!();

    println!("示例 10: 函数指针的线程安全性");
    example10_function_pointer();
    println!();

    println!("示例 11: 闭包的线程安全性");
    example11_closure_thread_safety();
    println!();

    println!("示例 12: 原子类型的 Send/Sync");
    example12_atomic_types();
    println!();

    println!("示例 13: 自定义类型的条件 Send/Sync 实现");
    example13_conditional_send_sync();
    println!();

    println!("示例 14: 检查类型是否实现 Send/Sync");
    example14_check_send_sync();
    println!();

    println!("示例 15: 使用 where 子句约束线程安全性");
    example15_where_clause();

    println!("\n=== 总结 ===");
    println!("Send 和 Sync 特点:");
    println!("  - Send: 可以跨线程转移所有权");
    println!("  - Sync: 可以跨线程共享引用");
    println!("  - 都是 marker traits（没有方法）");
    println!("  - 自动推导（通常不需要手动实现）");
    println!("  - unsafe impl 需要确保安全性不变式");
    println!("  - T: Send ⇒ &T: Sync");
    println!("  - T: Sync ⇒ &mut T: Sync");
    println!("  - 编译器在编译时检查线程安全性");
    println!("  - 违反安全性会有编译错误");
    println!("\n常见类型:");
    println!("  - 原子类型: Send + Sync");
    println!("  - 原始指针: 非 Send，非 Sync");
    println!("  - Rc: 非 Send，非 Sync");
    println!("  - Arc: Send(如果 T:Send), Sync(如果 T:Send+Sync)");
    println!("  - Mutex: Send(如果 T:Send), Sync(如果 T:Send)");
}
