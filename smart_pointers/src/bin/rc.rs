// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================ 
// Rc<T> - 引用计数智能指针
// ============================================================================ 
//
// Rc<T> (Reference Counted) 用于单线程场景下的共享所有权。
//
// 主要特点：
// 1. 允许多个所有者拥有同一份数据
// 2. 只能在单线程中使用（不是 Send + Sync）
// 3. 每次克隆增加引用计数，每次丢弃减少引用计数
// 4. 引用计数降为 0 时释放数据

use std::rc::Rc;

// ============================================================================ 
// 示例 1: 基本用法 - 共享所有权
// ============================================================================ 
#[derive(Debug)]
struct Data {
    value: i32,
}

#[allow(dead_code)]
fn example1_basic_rc() {
    let data = Rc::new(Data { value: 42 });
    println!("初始引用计数: {}", Rc::strong_count(&data));

    // 克隆 Rc，不克隆数据
    let data1 = Rc::clone(&data);
    println!("克隆后引用计数: {}", Rc::strong_count(&data));

    let data2 = Rc::clone(&data);
    println!("再次克隆后引用计数: {}", Rc::strong_count(&data));

    // 三个 Rc 指向同一份数据
    println!("data: {:?}", data);
    println!("data1: {:?}", data1);
    println!("data2: {:?}", data2);

    // data2 被丢弃
    drop(data2);
    println!("drop 后引用计数: {}", Rc::strong_count(&data));
}

// ============================================================================ 
// 示例 2: 构建共享数据结构 - 图
// ============================================================================ 
#[derive(Debug)]
struct Node {
    value: i32,
    // 使用 Rc<Node> 允许多个节点指向同一个子节点
    children: Vec<Rc<Node>>,
}

#[allow(dead_code)]
fn example2_shared_graph() {
    // 创建子节点，可以被多个父节点共享
    let shared_child = Rc::new(Node {
        value: 3,
        children: vec![],
    });

    // 创建父节点
    let parent1 = Rc::new(Node {
        value: 1,
        children: vec![Rc::clone(&shared_child)],
    });

    let parent2 = Rc::new(Node {
        value: 2,
        children: vec![Rc::clone(&shared_child)],
    });

    println!("图结构:");
    println!("  Parent1 (value: {:?}) -> Child (value: {:?})",
             parent1.value, parent1.children[0].value);
    println!("  Parent2 (value: {:?}) -> Child (value: {:?})",
             parent2.value, parent2.children[0].value);

    println!("子节点被引用次数: {}", Rc::strong_count(&shared_child));
}

// ============================================================================ 
// 示例 3: 与 RefCell 组合实现内部可变性
// ============================================================================ 
use std::cell::RefCell;

struct SharedCounter {
    value: RefCell<i32>,
}

#[allow(dead_code)]
fn example3_interior_mutability() {
    let counter = Rc::new(SharedCounter {
        value: RefCell::new(0),
    });

    // 多个所有者可以共享同一个 counter
    let counter1 = Rc::clone(&counter);
    let counter2 = Rc::clone(&counter);

    // 即使持有的是不可变引用，也可以修改内部值
    *counter1.value.borrow_mut() += 1;
    *counter2.value.borrow_mut() += 1;

    println!("计数器值: {}", counter.value.borrow());
    println!("引用计数: {}", Rc::strong_count(&counter));
}

// ============================================================================ 
// 示例 4: 双向链表（简化版）
// ============================================================================ 
#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

#[allow(dead_code)]
fn example4_linked_list() {
    // 创建节点
    let node3 = Rc::new(RefCell::new(ListNode {
        value: 3,
        next: None,
    }));

    let node2 = Rc::new(RefCell::new(ListNode {
        value: 2,
        next: Some(Rc::clone(&node3)),
    }));

    let node1 = Rc::new(RefCell::new(ListNode {
        value: 1,
        next: Some(Rc::clone(&node2)),
    }));

    // 遍历链表
    print!("链表: ");
    let mut current = Some(node1);
    while let Some(node) = current {
        print!("{} ", node.borrow().value);
        current = node.borrow().next.clone();
    }
    println!();
}

// ============================================================================ 
// 示例 5: 引用循环（内存泄漏风险）
// ============================================================================ 
struct CycleNode {
    value: i32,
    next: RefCell<Option<Rc<CycleNode>>>,
}

#[allow(dead_code)]
fn example5_reference_cycle() {
    // 创建节点
    let a = Rc::new(CycleNode {
        value: 1,
        next: RefCell::new(None),
    });

    let b = Rc::new(CycleNode {
        value: 2,
        next: RefCell::new(Some(Rc::clone(&a))),
    });

    // 创建循环引用
    *a.next.borrow_mut() = Some(Rc::clone(&b));

    // 此时引用计数永远不会降为 0，导致内存泄漏！
    println!("a 引用计数: {}", Rc::strong_count(&a));
    println!("b 引用计数: {}", Rc::strong_count(&b));

    println!("警告: 这是一个引用循环的例子，会导致内存泄漏！");
    println!("解决方案: 使用 Weak<T> 打破循环（见 weak.rs）");
}

// ============================================================================ 
// 示例 6: Rc::make_mut - 写时复制（Copy-on-Write）
// ============================================================================ 
#[allow(dead_code)]
fn example6_make_mut() {
    let mut data = Rc::new(vec![1, 2, 3]);
    println!("初始数据: {:?}", data);
    println!("初始引用计数: {}", Rc::strong_count(&data));

    // 克隆，共享数据
    let data2 = Rc::clone(&data);
    println!("克隆后引用计数: {}", Rc::strong_count(&data));

    // make_mut 会检查是否只有一个引用
    // 如果有多个引用，会先复制数据
    Rc::make_mut(&mut data).push(4);

    println!("修改后 data: {:?}", data);
    println!("修改后 data2: {:?}", data2);
    println!("修改后 data 引用计数: {}", Rc::strong_count(&data));
    println!("修改后 data2 引用计数: {}", Rc::strong_count(&data2));
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== Rc<T> 引用计数智能指针示例 ===\n");

    println!("示例 1: 基本用法 - 共享所有权");
    example1_basic_rc();
    println!();

    println!("示例 2: 构建共享数据结构 - 图");
    example2_shared_graph();
    println!();

    println!("示例 3: 与 RefCell 组合实现内部可变性");
    example3_interior_mutability();
    println!();

    println!("示例 4: 双向链表");
    example4_linked_list();
    println!();

    println!("示例 5: 引用循环（内存泄漏风险）");
    example5_reference_cycle();
    println!();

    println!("示例 6: make_mut - 写时复制");
    example6_make_mut();

    println!("\n=== 总结 ===");
    println!("Rc<T> 特点:");
    println!("  - 多所有者共享同一份数据");
    println!("  - 引用计数跟踪所有权");
    println!("  - 单线程专用（不是 Send + Sync）");
    println!("  - 与 RefCell 组合实现内部可变性");
    println!("  - 注意避免引用循环（使用 Weak<T>）");
}