use std::thread;

// 通用并发执行器：启动指定数量的线程并执行指定的逻辑
// 
// 参数说明：
// shared_data: 线程间共享的数据句柄（通常是 Arc<T>，但也可以是任何实现了 Clone + Send 的类型）
// count: 启动的线程数量
// task: 线程执行的具体逻辑闭包。该闭包接收两个参数：
//    1. 共享数据的线程本地副本 (T)
//    2. 当前线程的索引 (usize)
pub fn spawn_workers<T, F>(shared_data: T, count: usize, task: F)
where
    T: Send + Clone + 'static,
    F: Fn(T, usize) + Send + Sync + 'static + Clone,
{
    let mut handles = vec![];
    for i in 0..count {
        let data_clone = shared_data.clone();
        let task_clone = task.clone();
        let handle = thread::spawn(move || {
            task_clone(data_clone, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
