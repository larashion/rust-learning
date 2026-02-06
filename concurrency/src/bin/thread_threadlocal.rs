// ============================================================================
// 线程 (Threads) - 线程局部存储 (TLS)
// ============================================================================

use std::cell::Cell;
use std::thread;

thread_local! {
    // `Cell` 允许使用不可变引用修改内部数据
    // 惰性初始化
    static THREAD_LOCAL: Cell<i32> = const { Cell::new(0) };
}

fn main() {
    show_isolation();
}

fn show_isolation() {
    update_tls("主线程", 42);
    run_child();

    THREAD_LOCAL.with(|val| {
        println!("主线程: 最终值 = {} (验证隔离性)", val.get());
    });
}

fn run_child() {
    thread::spawn(|| {
        update_tls("子线程", 100);
    })
    .join()
    .expect("线程执行失败");
}

fn update_tls(label: &str, val: i32) {
    THREAD_LOCAL.with(|tls| {
        println!("{}: 初始 = {}", label, tls.get());
        tls.set(val);
        println!("{}: 修改 = {}", label, tls.get());
    });
}
