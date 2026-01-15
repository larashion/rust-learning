// ============================================================================ 
// 高级 Trait 技巧
// ============================================================================ 
//
// 1. 关联类型 (Associated Types): Trait 中的类型占位符 (如 Iterator::Item)
// 2. 默认泛型类型参数 (Default Generic Type Parameters): std::ops::Add<Rhs=Self>
// 3. Supertraits: 依赖于另一个 Trait (继承的效果)
// 4. Newtype 模式: 绕过孤儿规则

use std::fmt;

// ============================================================================ 
// 示例 1: 关联类型 vs 泛型 Trait
// ============================================================================ 
// 场景：我们需要一个包含"下一个元素"概念的 Trait。

// 方式 A: 泛型 (Generic)
// 缺点：对于同一个类型，你可以实现多次 IteratorGeneric<String>, IteratorGeneric<i32>...
// 导致每次使用时必须标注类型。
#[allow(dead_code)]
trait IteratorGeneric<T> {
    fn next(&mut self) -> Option<T>;
}

// 方式 B: 关联类型 (Associated Type)
// 优点：对于每个实现类型，Item 只能有一个确定的类型。编译器知道它是唯一的。
trait MyIterator {
    type Item; // 占位符
    fn next(&mut self) -> Option<Self::Item>;
}

#[allow(dead_code)]
struct Counter {
    count: u32,
}

impl MyIterator for Counter {
    type Item = u32; // 在实现时指定具体类型

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn example1_associated_types() {
    println!("--- 示例 1: 关联类型 ---");
    let mut c = Counter { count: 0 };
    while let Some(i) = c.next() {
        print!("{} ", i);
    }
    println!();
}

// ============================================================================ 
// 示例 2: 运算符重载 (默认泛型参数)
// ============================================================================ 
// std::ops::Add 定义如下：
// trait Add<Rhs=Self> { type Output; fn add(self, rhs: Rhs) -> Self::Output; }
// Rhs=Self 表示如果不指定 rhs 类型，默认就是加在这个类型自己身上。

use std::ops::Add;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// 正常情况：Point + Point
impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// 特殊情况：Point + i32 (改变了 Rhs)
impl Add<i32> for Point {
    type Output = Point;
    
    fn add(self, other: i32) -> Point {
        Point {
            x: self.x + other,
            y: self.y + other,
        }
    }
}

fn example2_operator_overloading() {
    println!("\n--- 示例 2: 运算符重载 ---");
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2; // invoke Add::add
    println!("Point + Point = {:?}", p3);

    let p4 = Point { x: 1, y: 1 };
    let p5 = p4 + 10; // invoke Add<i32>::add
    println!("Point + 10    = {:?}", p5);
}

// ============================================================================ 
// 示例 3: Supertraits (父 Trait)
// ============================================================================ 
// 这是一个"要求"而非"继承"。
// 如果你想实现 OutlinePrint，你就必须也实现 Display。

trait OutlinePrint: fmt::Display { // 要求 Self: fmt::Display
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("* {} *", output);
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for Point {} // Point 并没有实现 Display，这里会编译报错吗？
// 不会，因为我们在下面实现了 Display for Point。编译器会检查全局实现。

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn example3_supertraits() {
    println!("\n--- 示例 3: Supertraits ---");
    let p = Point { x: 10, y: 20 };
    p.outline_print();
}

// ============================================================================ 
// 示例 4: Newtype 模式
// ============================================================================ 
// 目标：我们想给 Vec<String> 实现 Display。
// 阻碍：Vec 是外部类型，Display 是外部 Trait。违反孤儿规则。
// 解决：创建一个元组结构体 (Wrapper) 把 Vec 包起来。Wrapper 是本地类型。

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn example4_newtype() {
    println!("\n--- 示例 4: Newtype 模式 ---");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("Wrapper displayed: {}", w);
}

fn main() {
    example1_associated_types();
    example2_operator_overloading();
    example3_supertraits();
    example4_newtype();
}
