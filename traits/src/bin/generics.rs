// ============================================================================
// Trait Bounds & 泛型约束
// ============================================================================
//
// 泛型允许代码对抽象类型工作，而 Trait Bounds 限制了这些类型必须具备的功能。
//
// 核心概念：
// 1. 泛型语法的 Trait Bound (`T: Trait`)
// 2. `where` 从句 (更清晰的写法)
// 3. 多重约束 (`T: Trait1 + Trait2`)
// 4. `impl Trait` 返回类型
// 5. 条件实现 (Blanket Implementations)

use std::fmt::Debug;
use std::fmt::Display;

// ============================================================================
// 示例 1: 简单的 Trait Bound
// ============================================================================
// 只有实现了 PartialOrd (可比较) 和 Copy (可复制，简化所有权) 的类型才能调用
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn example1_simple_bound() {
    println!("--- 示例 1: Trait Bound ---");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("最大的数字是: {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("最大的字符是: {}", result);
}

// ============================================================================
// 示例 2: where 从句
// ============================================================================
// 当约束太长时，签名会变得难以阅读。`where` 从句可以解耦签名和约束。

trait MyDisplay {
    fn display(&self);
}
trait MyDebug {
    fn debug(&self);
}
#[allow(dead_code)]
struct MyType;
impl MyDisplay for MyType {
    fn display(&self) {
        print!("Display");
    }
}
impl MyDebug for MyType {
    fn debug(&self) {
        print!("Debug");
    }
}

// 繁琐写法
#[allow(dead_code)]
fn some_function_clutter<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) {}

// 清晰写法 (where)
fn some_function_where<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("Called with: t={}, u={:?}", t, u);
}

fn example2_where_clause() {
    println!("\n--- 示例 2: where 从句 ---");
    some_function_where(&"Hello", &123);
}

// ============================================================================
// 示例 3: 返回 impl Trait
// ============================================================================
// 当你不想在函数签名中写出极其复杂的具体类型（如闭包或者迭代器链）时很有用。
// 注意：impl Trait 只能返回单一的具体类型。不能根据 if-else 返回不同的类型。

// 这里的返回值实际上是一个闭包类型，但我们只关心它实现了 Fn(i32) -> i32
fn returns_closure() -> impl Fn(i32) -> i32 {
    |x| x + 1
}

fn example3_impl_trait_return() {
    println!("\n--- 示例 3: 返回 impl Trait ---");
    let f = returns_closure();
    println!("Closure result: {}", f(1));
}

// ============================================================================
// 示例 4: 条件实现 (Blanket Implementation)
// ============================================================================
// Rust 标准库中非常强大的模式：为所有实现了 Trait A 的类型自动实现 Trait B。
// 例如：impl<T: Display> ToString for T { ... }
// 这就是为什么任何实现了 Display 的类型都能调用 .to_string()。

trait Printable {
    fn print_me(&self);
}

// 为所有实现了 Display 的类型自动实现 Printable
impl<T: Display> Printable for T {
    fn print_me(&self) {
        println!("Printable: {}", self);
    }
}

fn example4_blanket_impl() {
    println!("\n--- 示例 4: Blanket Implementation ---");
    let s = 123;
    // i32 实现了 Display，所以它自动获得了 Printable 的能力
    s.print_me();

    let str = "Hello";
    str.print_me();
}

fn main() {
    example1_simple_bound();
    example2_where_clause();
    example3_impl_trait_return();
    example4_blanket_impl();
}
use std::ops::Add;

struct MyStruct<T> {
    val: T,
}

impl<T> Add for MyStruct<T>
where
    T: Add<Output = T>, // 前提是盒子里的内容相加后类型不变。
{
    type Output = Self; //防变异

    fn add(self, other: Self) -> Self::Output {
        MyStruct {
            val: self.val + other.val,
        }
    }
}
