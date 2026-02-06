use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== 非阻塞与超时 ===");
    let (tx, rx) = mpsc::channel();

    println!("try_recv: {:?}", rx.try_recv());

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        tx.send("迟到的消息").unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(90)) {
        Ok(msg) => println!("超时内收到: {}", msg),
        Err(_) => println!("等待超时!"),
    }
}
