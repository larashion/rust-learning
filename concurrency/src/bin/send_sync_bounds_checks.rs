use std::sync::Arc;
use std::thread;

/// 1. 使用泛型约束进行显式检查
///
/// 演示如何通过辅助函数在静态编译期检查类型是否实现 Send/Sync
fn example_explicit_bounds() {
    println!("--- 示例 1: 使用泛型约束进行显式检查 ---");

    fn is_send<T: Send>(_value: &T) {
        println!("类型实现过 Send");
    }

    fn is_sync<T: Sync>(_value: &T) {
        println!("类型实现了 Sync");
    }

    let data = 42;
    is_send(&data);
    is_sync(&data);
}

/// 2. 在线程派发中使用 Trait Bounds
///
/// 演示在实际 API（如 thread::spawn）中，编译器如何利用 Bounds 确保安全
fn example_spawn_bounds() {
    println!("\n--- 示例 2: 在线程派发中使用 Trait Bounds ---");

    fn spawn_with_check<T: Send + 'static>(data: T) {
        let _handle = thread::spawn(move || {
            // data 在这里可以使用
            let _ = data;
        });
        println!("带有 Send trait bounds 的数据才不会导致编译错误");
    }
    // ✅ Arc 实现了 Send
    spawn_with_check(Arc::new(5));

    // ✅ Vec 实现了 Send
    spawn_with_check(vec![1, 2, 3]);

    /*
    ❌ 演示编译错误 (取消注释将无法编译):

    // Rc 没有实现 Send，不能在线程间转移
    spawn_with_check(Rc::new(5));

    // &Vec 虽然满足 Send，但不满足 'static (生命周期不足)
    let v = vec![1, 2, 3];
    spawn_with_check(&v);
    */

    println!("(Rc 和引用无法通过编译)");
}

fn main() {
    println!("=== Send 和 Sync 的约束检查 ===");

    // 示例 1
    example_explicit_bounds();

    // 示例 2
    example_spawn_bounds();
}
