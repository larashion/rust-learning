// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================
// Lifetimes - 结构体与方法
// ============================================================================
//
// 进阶场景：
// 1. 结构体中包含引用：必须标注生命周期，因为结构体实例不能活得比它引用的数据更久。
// 2. 生命周期省略规则（Elision Rules）：编译器为了方便，自动补全某些常见模式的标注。
// 3. 'static 生命周期：特殊的生命周期，贯穿整个程序运行期。
struct MyString<'a> {
    text: &'a str,
}
impl<'a> MyString<'a> {
    fn modify_data(&mut self) {
        self.text = "Modified data";
    }
}

// ============================================================================
// 示例 1: 结构体中的生命周期
// ============================================================================
// 这个结构体持有一个字符串切片（引用）。
// 这意味着 ImportantExcerpt 的实例不能比它引用的 `part` 活得更久。
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn example1_struct_lifetimes() {
    println!("--- 示例 1: 结构体持有引用 ---");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // i 持有 novel 的一部分引用
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // novel 必须在 i 销毁之前一直有效
    println!("提取的片段: {:?}", i);
}

// ============================================================================
// 示例 2: 方法中的生命周期 & 省略规则
// ============================================================================
impl<'a> ImportantExcerpt<'a> {
    // 省略规则 1：每个引用参数都有自己的生命周期参数。
    // 省略规则 2：如果只有一个输入生命周期，它会被赋给所有输出生命周期。
    // 省略规则 3：如果有 &self 或 &mut self，那么 self 的生命周期会赋给所有输出生命周期。

    // 这里应用了规则 1：不需要给 level 标注
    #[allow(dead_code)]
    fn level(&self) -> i32 {
        3
    }

    // 这里应用了规则 3：返回值的生命周期自动被推导为与 &self 一致
    // 也就是返回值的生命周期来自于结构体实例本身（或者说结构体引用的数据）
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn example2_method_lifetimes() {
    println!("\n--- 示例 2: 方法与省略规则 ---");

    let novel = String::from("Rust is hard but worth it.");
    let first_part = novel.split_whitespace().next().unwrap();
    let i = ImportantExcerpt { part: first_part };

    let s = i.announce_and_return_part("Method called!");
    println!("Return value: {}", s);
}

// ============================================================================
// 示例 3: 'static 生命周期
// ============================================================================
fn example3_static_lifetime() {
    println!("\n--- 示例 3: 'static 生命周期 ---");

    // 1. 字符串字面量拥有 'static 生命周期，因为它们被硬编码在程序二进制文件中
    let s: &'static str = "I have a static lifetime.";
    println!("{}", s);

    // 2. Trait Bound 中的 'static
    // <T: 'static> 意味着 T 不包含任何非静态的引用（可以持有所有权，或者只持有 'static 引用）
    fn verify_static<T: 'static>(_t: T) {
        println!("Type checks out as static context safe.");
    }

    let owned_string = String::from("owned");
    verify_static(owned_string); // OK: String 拥有数据，不借用任何临时的东西

    // let temp = "temp".to_string();
    // let reference = &temp;
    // verify_static(reference); // ❌ 错误: 引用不是 'static 的
}

// ============================================================================
// 示例 4: 综合应用 - 泛型、Trait Bound 和 生命周期
// ============================================================================
use std::fmt::Display;

// 这里的 'a 应用于 x, y 和返回值
// T 必须实现 Display trait
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    example1_struct_lifetimes();
    example2_method_lifetimes();
    example3_static_lifetime();

    println!("\n--- 示例 4: 综合应用 ---");
    longest_with_an_announcement("abcd", "xyz", "Testing generics");
}
