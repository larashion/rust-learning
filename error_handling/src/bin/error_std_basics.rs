// ============================================================================
// Error Handling
// ============================================================================
//
// Rust 没有异常（Exception），而是使用 Result<T, E> 和 Option<T> 枚举来处理错误和空值。
//
// 核心原则：
// 1. 显式处理所有可能失败的情况。
// 2. 尽量避免使用 unwrap() / expect()，除非你确定那里绝对不会出错（或者你在写原型/测试）。
// 3. 使用 ? 运算符传播错误。
#![allow(clippy::unnecessary_literal_unwrap)]
use std::fs::File;
use std::io::{self, Read};

// ============================================================================
// 示例 1: Result 与 ? 运算符
// ============================================================================

// 优雅写法（推荐）：使用 ? 运算符
// 如果 Result 是 Ok，? 表达式取值；
// 如果 Result 是 Err，? 会立即从当前函数返回该 Err。
fn read_from_file() -> Result<String, io::Error> {
    let mut s = String::new();
    // 链式调用
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

fn example1_propagation() {
    println!("--- 示例 1: 错误传播 ---");
    match read_from_file() {
        Ok(s) => println!("读到内容: {}", s),
        Err(e) => println!("读取失败 (预期内): {}", e),
    }
}
// ============================================================================
// 示例 2: Option/Result Combinators 避免 match hell。
// ============================================================================
fn example2_combinators() {
    println!("\n--- 示例 2: Result/Option 组合 ---");

    let maybe_number: Option<i32> = Some(5);
    // let maybe_number: Option<i32> = None;

    let doubled = maybe_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);

    let value = maybe_number.unwrap_or(0);
    println!("Value: {}", value);

    // 惰性求值
    let value_lazy = maybe_number.unwrap_or_else(|| {
        println!("计算默认值...");
        0
    });
    println!("Value Lazy: {}", value_lazy);

    let s = "4";
    let result = s
        .parse::<i32>()
        .ok() // Result -> Option
        .and_then(|n| if n >= 0 { Some(f64::from(n)) } else { None }) // 转换为小数并检查负数
        .map(|n| n.sqrt()); // 开方

    println!("Sqrt calculation: {:?}", result);
}

// ============================================================================
// 示例 3:  Boilerplate 自定义错误类型
// ============================================================================
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError: {}", self.details)
    }
}

// 实现 Error trait (这是标准做法，以便能与其他错误处理库兼容)
impl std::error::Error for MyError {}

fn do_something_risky() -> Result<(), MyError> {
    Err(MyError::new("Something went wrong!"))
}

fn example3_custom_error() {
    println!("\n--- 示例 3: 自定义错误 ---");
    match do_something_risky() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Caught custom error: {}", e),
    }
}

fn main() {
    example1_propagation();
    example2_combinators();
    example3_custom_error();
}
