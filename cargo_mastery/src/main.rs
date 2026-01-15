// ============================================================================ 
// Binary Entry Point
// ============================================================================ 
// 这里的 main.rs 是二进制 crate 的根。
// 它必须通过 crate name 来引用同目录下的 library crate。

use cargo_mastery::eat_at_restaurant;
use cargo_mastery::kitchen;

fn main() {
    println!("=== Cargo Modules Demo ===");
    
    // 调用库中的公共函数
    eat_at_restaurant();

    // 直接使用库中的模块
    println!("\nDirectly accessing library modules:");
    kitchen::order_breakfast();
}
