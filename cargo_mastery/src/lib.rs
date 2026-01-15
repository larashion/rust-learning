// ============================================================================ 
// 模块系统 (Module System) 总入口
// ============================================================================ 
// 在 Rust 中，模块树的结构必须明确定义。
// lib.rs 或 main.rs 是 crate 的根 (Crate Root)。

// 1. 声明子模块
// 编译器会去寻找同名文件 (kitchen.rs) 或同名目录下的 mod.rs (garden/mod.rs)
pub mod kitchen;
pub mod garden;

// 2. 也是一个模块 (内联定义)
// 通常用于简单的工具函数或测试
mod utilities {
    pub fn help() {
        println!("Utilities: helping out...");
    }
}

// 3. 绝对路径与相对路径演示
pub fn eat_at_restaurant() {
    println!("--- 模块路径演示 ---");
    
    // 绝对路径 (Absolute Path): 从 crate root 开始
    crate::garden::vegetables::harvest();

    // 相对路径 (Relative Path): 从当前模块开始
    self::kitchen::order_breakfast(); // self 可以省略

    // 使用 use 引入的路径
    use crate::kitchen::Appetizer;
    let _order1 = Appetizer::Soup;
    let _order2 = Appetizer::Salad;
}

// ============================================================================ 
// 单元测试 (Unit Tests)
// ============================================================================ 
// 1. 位置: 通常位于源码文件的底部。
// 2. 访问权限: 可以访问父模块的私有函数 (private functions)。
// 3. 标注: 使用 #[cfg(test)] 只有在 cargo test 时才编译。

// 一个私有辅助函数 (Private Helper)
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*; // 引入父模块的所有内容

    #[test]
    fn test_internal_logic() {
        // 关键点: 单元测试可以测试私有函数！
        assert_eq!(internal_adder(2, 2), 4);
    }

    #[test]
    fn test_public_api() {
        crate::eat_at_restaurant(); // 也可以测公开 API
    }
}
