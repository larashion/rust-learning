use learning_concurrency::spawn_workers;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
struct Database {
    data: Vec<String>,
}

impl Deref for Database {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

fn demo_complex_struct_with_rwlock() {
    let db = Arc::new(RwLock::new(Database { data: vec![] }));

    println!("--- 开始写入 ---");
    // 调用通用的并发执行器，传入 Arc 的克隆
    spawn_workers(Arc::clone(&db), 3, |db: Arc<RwLock<Database>>, i| {
        let mut guard = db.write().unwrap();
        guard.push(format!("数据 {}", i));
        println!("写者: 添加数据 {}", i);
    });

    println!("\n--- 开始读取 ---");
    spawn_workers(Arc::clone(&db), 3, |db: Arc<RwLock<Database>>, i| {
        let guard = db.read().unwrap();
        println!("读者 {:?}: {:?}", i, *guard);
    });
}

fn main() {
    println!("=== RwLock<T> 与 泛型并发抽象 示例 ===");
    demo_complex_struct_with_rwlock();
}
