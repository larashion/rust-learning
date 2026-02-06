use learning_concurrency::spawn_workers;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct SharedData {
    counter: i32,
    values: Vec<i32>,
}

/// 演示使用 Arc<Mutex<T>> 共享和修改复杂类型
fn run_complex_data_sharing() {
    println!("--- 共享复杂类型演示 ---");

    let data = Arc::new(Mutex::new(SharedData {
        counter: 0,
        values: vec![],
    }));

    // 使用通用并发执行器 spawn_workers
    spawn_workers(Arc::clone(&data), 5, |data: Arc<Mutex<SharedData>>, i| {
        let mut guard = data.lock().unwrap();
        guard.counter += 1;
        guard.values.push(i as i32);
        println!(
            "线程 {}: counter={}, values={:?}",
            i, guard.counter, guard.values
        );
    });

    println!("\n[结果] 最终共享数据: {:?}", *data.lock().unwrap());
}

fn main() {
    println!("=== Mutex 与 Arc 组合 ===\n");

    run_complex_data_sharing();

    println!("\n============");
}
