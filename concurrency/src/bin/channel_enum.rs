use std::sync::mpsc;
use std::thread;

/// 定义消息类型枚举
#[derive(Debug)]
enum Task {
    Compute(i32, i32),
    Log(String),
    Quit,
}

/// 模拟一个处理不同任务的逻辑
fn handle_tasks(rx: mpsc::Receiver<Task>) {
    println!("工作线程: 已启动");

    // 使用 while let 接收消息
    while let Ok(task) = rx.recv() {
        match task {
            Task::Compute(a, b) => {
                println!("工作线程: 收到计算任务，结果为 {}", a + b);
            }
            Task::Log(msg) => {
                println!("工作线程: 记录日志 -> \"{}\"", msg);
            }
            Task::Quit => {
                println!("工作线程: 收到退出指令");
                break;
            }
        }
    }
    println!("工作线程: 已停止接收消息");
}

fn main() {
    println!("=== Channel 传递枚举消息 ===");

    let (tx, rx) = mpsc::channel();

    let handle = thread::spawn(move || {
        handle_tasks(rx);
    });

    let send_task = |task: Task| {
        if tx.send(task).is_err() {
            eprintln!("发送失败: 通道可能已关闭");
        }
    };

    // 发送任务
    send_task(Task::Log(String::from("系统初始化...")));
    send_task(Task::Compute(10, 20));
    send_task(Task::Log(String::from("正在执行中间步骤...")));
    send_task(Task::Compute(100, 200));

    // 发送退出指令
    send_task(Task::Quit);

    // 等待子线程结束
    if handle.join().is_err() {
        eprintln!("子线程发生 panic");
    }

    println!("主线程: 演示结束");
}
