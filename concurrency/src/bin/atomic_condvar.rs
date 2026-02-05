use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

fn example_condvar() {
    // 数据对：锁 + 条件变量
    let pair = Arc::new((Mutex::new(false), Condvar::new())); // false 代表"空"，true 代表"满"
    let pair2 = Arc::clone(&pair);
    // 生产者
    let producer = thread::spawn(move || {
        let (lock, cvar) = &*pair;

        for i in 1..5 {
            let mut started = lock.lock().unwrap();

            while *started {
                started = cvar.wait(started).unwrap();
            }

            // 2. 醒来发现 false，生产数据
            println!("生产者: 生产数据 {}", i);
            thread::sleep(Duration::from_millis(50)); // 模拟耗时

            // 3. 修改状态为 true (满)
            *started = true;

            // 4. 通知消费者 (饭做好了)
            cvar.notify_one();
        }
    });
    // 消费者
    let consumer = thread::spawn(move || {
        let (lock, cvar) = &*pair2;

        for i in 1..5 {
            let mut started = lock.lock().unwrap();
            // 1. 等待数据：只要是 false (没数据)，就睡着
            while !*started {
                started = cvar.wait(started).unwrap();
            }

            // 2. 醒来发现 true，消费数据
            println!("消费者: 消费数据 {}", i);

            // 3. 修改状态为 false (空)
            *started = false;

            // 4. 通知生产者
            cvar.notify_one();
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

fn main() {
    println!("=== 示例: 条件变量 (Condvar) ===");
    example_condvar();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_condvar_notification() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);

        thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            *started = true;
            cvar.notify_one();
        });

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        // 如果没有收到通知，wait 会一直阻塞，导致测试超时
        while !*started {
            started = cvar.wait(started).unwrap();
        }
        assert!(*started);
    }
}
