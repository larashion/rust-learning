#![allow(clippy::unnecessary_literal_unwrap)]
// ============================================================================
// Error Handling - 标准库基础
// ============================================================================
//
// Rust 没有异常（Exception），而是使用 Result<T, E> 和 Option<T> 枚举来处理错误和空值。
//
// 核心原则：
// 1. 显式处理所有可能失败的情况。
// 2. 尽量避免使用 unwrap() / expect()，除非你确定那里绝对不会出错（或者你在写原型/测试）。
// 3. 使用 ? 运算符传播错误。

use std::fs::File;
use std::io::{self, Read};

// ============================================================================
// 示例 1: Result 与 ? 运算符
// ============================================================================
// 传统写法（繁琐）
#[allow(dead_code)]
fn read_username_from_file_manual() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = f?;

    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 优雅写法（推荐）：使用 ? 运算符
// 如果 Result 是 Ok，? 表达式取值；
// 如果 Result 是 Err，? 会立即从当前函数返回该 Err。
fn read_username_from_file_question_mark() -> Result<String, io::Error> {
    let mut s = String::new();
    // 链式调用
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 极简写法：使用 std::fs::read_to_string
#[allow(dead_code)]
fn read_username_shortest() -> Result<String, io::Error> {
    std::fs::read_to_string("hello.txt")
}

fn example1_propagation() {
    println!("--- 示例 1: 错误传播 ---");
    // 这里因为文件不存在，预期会报错
    match read_username_from_file_question_mark() {
        Ok(s) => println!("读到内容: {}", s),
        Err(e) => println!("读取失败 (预期内): {}", e),
    }
}

// ============================================================================
// 示例 2: 组合子 (Combinators) - map, and_then, unwrap_or
// ============================================================================
// 避免 match hell 的利器。

fn example2_combinators() {
    println!("\n--- 示例 2: Result/Option 组合子 ---");

    let maybe_number: Option<i32> = Some(5);
    // let maybe_number: Option<i32> = None;

    // map: 如果是 Some，就对值进行转换；如果是 None，保持 None
    let doubled = maybe_number.map(|n| n * 2);
    println!("Doubled: {:?}", doubled);

    // unwrap_or: 如果是 Some 取值，否则返回默认值
    let value = maybe_number.unwrap_or(0);
    println!("Value: {}", value);

    // unwrap_or_else: 只有在 None 时才执行闭包（惰性求值，适合开销大的默认值）
    let value_lazy = maybe_number.unwrap_or_else(|| {
        println!("计算默认值...");
        0
    });
    println!("Value Lazy: {}", value_lazy);

    // and_then: 用于链式操作，其中每一步都可能返回 Option/Result
    // 场景：解析字符串 -> 转数字 -> 开方 (不允许负数)
    let s = "4";
    let result = s
        .parse::<i32>()
        .ok() // Result -> Option
        .and_then(|n| if n >= 0 { Some(f64::from(n)) } else { None }) // 检查负数
        .map(|n| n.sqrt()); // 计算开方

    println!("Sqrt calculation: {:?}", result);
}

// ============================================================================
// 示例 3: 自定义错误类型 (基础版)
// ============================================================================
use std::fmt;

#[derive(Debug)]
struct MyError {
    details: String,
}

impl MyError {
    fn new(msg: &str) -> MyError {
        MyError {
            details: msg.to_string(),
        }
    }
}

// 实现 Display trait 以便打印
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
