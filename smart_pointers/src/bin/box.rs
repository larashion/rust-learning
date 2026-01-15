#![allow(unused)]
#![allow(clippy::boxed_local)]
// ============================================================================ 
// Box<T> - 堆分配的智能指针
// ============================================================================ 
//
// Box<T> 是最简单的智能指针，用于在堆上分配数据。
//
// 主要用途：
// 1. 在编译时未知大小的类型（递归类型）
// 2. 转移大量数据的所有权，避免复制
// 3. 实现特征（trait 对象）

// ============================================================================ 
// 示例 1: 在堆上分配数据
// ============================================================================ 
fn example1_basic_box() {
    let x = 5; // x 存储在栈上
    let y = Box::new(5); // y 是指向堆上值 5 的 Box 指针

    println!("x = {}, y = {}", x, y);
    println!("解引用 y: {}", *y);

    // Box 实现了 Deref trait，所以可以像普通引用一样使用
    // y 自动解引用，可以直接比较
    assert_eq!(x, *y);
}

// ============================================================================ 
// 示例 2: 递归类型 - 链表
// ============================================================================ 

#[derive(Debug)]
enum List {
    // 不使用 Box，递归类型会无限大，因为编译器不知道需要多少空间
    // Cons(i32, List), // 编译错误！

    // 使用 Box，Cons 的大小变为固定：一个 i32 + 一个指针
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn example2_recursive_type() {
    // 现在可以创建链表了
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("递归链表: {:?}", list);
}

// ============================================================================ 
// 示例 3: 转移大数据的所有权
// ============================================================================ 
#[derive(Debug)]
struct BigData {
    data: Vec<u8>, // 假设是一个很大的数据
}

fn example3_transfer_ownership() {
    // 创建大数据
    let big_data = BigData {
        data: vec![0; 1024 * 1024], // 1MB 数据
    };

    // 直接传递会移动整个结构体
    // 使用 Box 可以只移动指针，数据仍在堆上不动
    let boxed_data = Box::new(big_data);

    // 将 Box 传递给函数，只复制指针（8字节）
    process_big_data(boxed_data);
}

fn process_big_data(data: Box<BigData>) {
    println!("处理大数据，大小: {} 字节", data.data.len());
    // data 在这里被 drop
}

// ============================================================================ 
// 示例 4: Deref 和 DerefMut trait
// ============================================================================ 
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// 实现 Deref trait，允许像引用一样使用
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 实现 DerefMut trait，允许像可变引用一样使用
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn example4_deref_trait() {
    let x = 5;
    let y = MyBox::new(x);

    // Deref 强制转换：&MyBox<T> 自动转换为 &T
    assert_eq!(5, *y);
    assert_eq!(5, *y); // 这会自动调用 *(y.deref())

    // 可以直接调用 String 的方法，因为 MyBox<String> 可以转换为 &String
    let s = MyBox::new(String::from("hello"));
    println!("长度: {}", s.len()); // 自动解引用调用 len()
}

// ============================================================================ 
// 示例 5: Trait 对象（动态分发）
// ============================================================================ 
trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汪汪！");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("喵喵！");
    }
}

fn example5_trait_object() {
    // 使用 Box<dyn Animal> 创建 trait 对象
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog),
        Box::new(Cat),
    ];

    for animal in animals {
        animal.make_sound(); // 动态分发
    }
}

// ============================================================================ 
// 示例 6: Box 的内存布局
// ============================================================================ 
fn example6_memory_layout() {
    // Box 只是一个指针，在栈上占用指针大小的空间
    use std::mem;

    let x = Box::new(42);
    println!("Box 在栈上占用: {} 字节", mem::size_of_val(&x));
    println!("Box 指向的值在堆上占用: {} 字节", mem::size_of::<i32>());

    // Box 被丢弃时，堆上的内存也会被释放
    // 这得益于 Rust 的 RAII（Resource Acquisition Is Initialization）
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== Box<T> 智能指针示例 ===\n");

    println!("示例 1: 基本用法");
    example1_basic_box();
    println!();

    println!("示例 2: 递归类型");
    example2_recursive_type();
    println!();

    println!("示例 3: 转移大数据所有权");
    example3_transfer_ownership();
    println!();

    println!("示例 4: Deref trait");
    example4_deref_trait();
    println!();

    println!("示例 5: Trait 对象");
    example5_trait_object();
    println!();

    println!("示例 6: 内存布局");
    example6_memory_layout();

    println!("\n=== 总结 ===");
    println!("Box<T> 特点:");
    println!("  - 单一所有权，不可复制");
    println!("  - 自动释放内存（Drop）");
    println!("  - 实现了 Deref trait");
    println!("  - 常用于递归类型和 trait 对象");
}