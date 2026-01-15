// ============================================================================ 
// Rust 2024 (1.75+) 新特性: Async Fn in Traits
// ============================================================================ 
// 
// 在 Rust 1.75 之前，Trait 中不能包含 async fn。
// 那个时候，我们必须使用 #[async_trait] 宏（它会把返回值装箱成 BoxFuture）。
//
// 现在（2024 Edition 时代），这是原生支持的！
// 优点：
// 1. 无需额外依赖 (no async-trait)
// 2. 更少的内存分配 (no Box per call in static dispatch)
// 3. 更好的编译器支持

use std::time::Duration;
use tokio::time::sleep;

// ============================================================================ 
// 1. 定义包含 async fn 的 Trait (新语法)
// ============================================================================ 
trait AsyncService {
    // 以前这里会报错，现在这完全合法！
    async fn fetch_data(&self, id: u32) -> String;
    
    // 也可以这由默认实现
    async fn default_action(&self) {
        println!("Default async action...");
        sleep(Duration::from_millis(50)).await;
    }
}

// ============================================================================ 
// 2. 实现 Trait
// ============================================================================ 
struct DatabaseService;
struct NetworkService;

impl AsyncService for DatabaseService {
    async fn fetch_data(&self, id: u32) -> String {
        println!("[DB] Querying ID: {}", id);
        // 模拟 I/O
        sleep(Duration::from_millis(100)).await;
        format!("User_{}", id)
    }
}

impl AsyncService for NetworkService {
    async fn fetch_data(&self, id: u32) -> String {
        println!("[Net] Fetching URL for ID: {}", id);
        sleep(Duration::from_millis(300)).await;
        format!("Response_{}", id)
    }
}

// ============================================================================ 
// 3. 使用 Trait (静态分发)
// ============================================================================ 
// 这种方式是零开销的，没有额外的 Box 分配。
async fn process_request<S: AsyncService>(service: &S, id: u32) {
    let data = service.fetch_data(id).await;
    println!("Processed: {}", data);
    service.default_action().await;
}

// ============================================================================ 
// 4. 动态分发 (Dynamic Dispatch) 与对象安全
// ============================================================================ 
// ⚠️ 注意：带有 async fn 的 Trait 默认不是“对象安全”(Object Safe) 的。
// 这意味着你不能直接使用 Box<dyn AsyncService>。
// 
// 为什么？因为每个实现返回的 Future 类型都不同，大小也不确定。
// 只有当返回值确定大小时，才能放入 vtable。
//
// 解决方案 (1.75+): 使用 Send + Sync 约束，通常需要手动 Box，或者继续使用 async-trait 宏用于 dyn 场景。
// 未来 (Rust 2024+): 可能会有 `-> impl Future` 在 trait object 里的自动支持。

// 演示：目前直接使用 dyn AsyncService 会有困难。
// 为了演示方便，我们这里只展示 Static Dispatch。

#[tokio::main]
async fn main() {
    println!("=== Rust 2024: Native Async Traits Demo ===");

    let db = DatabaseService;
    let net = NetworkService;

    println!("\n--- Static Dispatch (Zero Overhead) ---");
    process_request(&db, 101).await;
    process_request(&net, 202).await;
    
    println!("\nSuccess! No #[async_trait] macro used.");
}
