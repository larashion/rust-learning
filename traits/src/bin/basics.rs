// ============================================================================
// Trait 基础 - 定义共有行为
// ============================================================================
//
// Trait (特质) 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。
//
// 核心概念：
// 1. 定义 Trait (trait 关键字)
// 2. 为类型实现 Trait (impl Trait for Type)
// 3. 默认实现 (Default Implementations)
// 4. 作为参数使用 (impl Trait 语法)

// 1. 定义 Trait
trait Summary {
    // 必须实现的方法
    fn summarize_author(&self) -> String;

    // 默认实现的方法
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 2. 定义具体的类型
#[allow(dead_code)]
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

#[allow(dead_code)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// 3. 为类型实现 Trait
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }

    // 重写默认实现
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // 使用默认的 existing summarize() 实现
}

// 4. 使用 Trait 作为参数
fn notify(item: &impl Summary) {
    println!("Breaking News! {}", item.summarize());
}

fn example1_basics() {
    println!("--- 示例 1: Trait 基础 ---");
    let article = NewsArticle {
        headline: String::from("Rust 赢得最受喜爱编程语言奖"),
        location: String::from("全球"),
        author: String::from("Stack Overflow"),
        content: String::from("Rust 连续多年霸榜..."),
    };

    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Hello, world!"),
        reply: false,
        retweet: false,
    };

    // 调用 trait 方法
    println!("Article Summary: {}", article.summarize());
    println!("Tweet Summary: {}", tweet.summarize());

    // 多态调用
    notify(&article);
    notify(&tweet);
}

// ============================================================================
// 示例 2: 孤儿规则 (Orphan Rule) 演示
// ============================================================================
// 术语: Orphan Rule (孤儿规则)
// 目的: 维护一致性 (Coherence) —— 确保对于任何给定的 Type 和 Trait，全局只有唯一的实现。
//
// 规则详情:
// 如果你想为类型 T 实现 Trait A，那么 T (类型) 或 A (Trait) 必须至少有一个是在当前 Crate 中定义的。
//
// 为什么叫 "孤儿" (Orphan)?
// 如果 Trait 是外部的 (Upstream)，Type 也是外部的，那么这个 `impl` 就既不属于 Trait 的作者，
// 也不属于 Type 的作者。编译器不知道该把这个实现放在哪里，它就像个"孤儿"一样无人认领。
// 如果允许这样做，两个不同的 Crate 可能会为同一个外部类型实现同一个外部 Trait，导致冲突。

// 场景 A: 本地 Trait, 外部类型 -> OK
// 因为 Trait 是我们定义的，我们拥有解释权。
#[allow(dead_code)]
trait LocalTrait {
    fn hello(&self);
}
impl LocalTrait for String {
    fn hello(&self) {
        println!("Hello from extension trait on String: {}", self);
    }
}

// 场景 B: 外部 Trait, 本地类型 -> OK
// use std::fmt::Display;
// struct LocalStruct;
// impl Display for LocalStruct { ... }

// 场景 C: 外部 Trait, 外部类型 -> ❌ Compile Error
// impl Display for String { ... }

fn example2_orphan_rule() {
    println!("\n--- 示例 2: 扩展外部类型 (Extension Trait) ---");
    let s = String::from("World");
    s.hello();
}

fn main() {
    example1_basics();
    example2_orphan_rule();
}
