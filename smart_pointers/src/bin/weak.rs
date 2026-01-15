// #![allow(unused)] // Cleaned up: Removed global suppression
// ============================================================================ 
// Weak<T> - 弱引用智能指针
// ============================================================================ 
//
// Weak<T> 用于打破引用循环，防止内存泄漏。
//
// 主要特点：
// 1. 不持有数据的所有权
// 2. 不增加强引用计数
// 3. 可以通过 upgrade() 尝试升级为 Rc/Arc
// 4. 如果强引用计数为 0，upgrade() 返回 None
// 5. 必须与 Rc 或 Arc 配合使用

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::Arc;
use std::thread;

// ============================================================================ 
// 示例 1: 基本 Weak 用法
// ============================================================================ 
fn example1_basic_weak() {
    let strong = Rc::new(42);
    let weak = Rc::downgrade(&strong);

    println!("强引用计数: {}", Rc::strong_count(&strong));
    println!("弱引用计数: {}", Rc::weak_count(&strong));

    // 通过 Weak 获取数据
    match weak.upgrade() {
        Some(arc) => println!("升级成功，值: {}", *arc),
        None => println!("升级失败，数据已被释放"),
    }

    // 丢弃强引用
    drop(strong);

    // 再次尝试升级
    match weak.upgrade() {
        Some(arc) => println!("升级成功，值: {}", *arc),
        None => println!("升级失败：数据已被释放"),
    }

    println!("强引用计数（drop 后）: 0");
    println!("弱引用计数（drop 后）: {}", weak.weak_count()); // weak 仍然存在
}

// ============================================================================ 
// 示例 2: 避免引用循环 - 树结构
// ============================================================================ 
#[derive(Debug)]
struct TreeNode {
    value: i32,
    // 子节点使用强引用
    children: Vec<Rc<RefCell<TreeNode>>>, 
    // 父节点使用弱引用，避免循环
    parent: RefCell<Weak<RefCell<TreeNode>>>, 
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode { 
            value,
            children: vec![],
            parent: RefCell::new(Weak::new()),
        }))
    }

    fn add_child(parent: &Rc<RefCell<TreeNode>>, child: Rc<RefCell<TreeNode>>) {
        // 设置子节点的父节点（弱引用）
        *child.borrow_mut().parent.borrow_mut() = Rc::downgrade(parent);
        // 添加到父节点的子节点列表（强引用）
        parent.borrow_mut().children.push(child);
    }
}

fn example2_tree_without_cycles() {
    let root = TreeNode::new(1);
    let child1 = TreeNode::new(2);
    let child2 = TreeNode::new(3);

    TreeNode::add_child(&root, child1.clone());
    TreeNode::add_child(&root, child2.clone());

    println!("树结构:");
    println!("根节点值: {}", root.borrow().value);
    println!("根节点强引用计数: {}", Rc::strong_count(&root));

    // 子节点可以访问父节点
    if let Some(parent) = child1.borrow().parent.borrow().upgrade() {
        println!("子节点的父节点值: {}", parent.borrow().value);
    }

    // 树可以被正常释放，没有循环引用
    drop(root);
    drop(child1);
    drop(child2);

    println!("所有节点已被释放（无内存泄漏）");
}

// ============================================================================ 
// 示例 3: 有循环引用的树（对比）
// ============================================================================ 
struct CycleNode {
    value: i32,
    parent: RefCell<Option<Rc<RefCell<CycleNode>>>>,
    children: Vec<Rc<RefCell<CycleNode>>>, 
}

fn example3_tree_with_cycles() {
    let root = Rc::new(RefCell::new(CycleNode { 
        value: 1,
        parent: RefCell::new(None),
        children: vec![],
    }));

    let child = Rc::new(RefCell::new(CycleNode { 
        value: 2,
        parent: RefCell::new(Some(Rc::clone(&root))),
        children: vec![],
    }));

    root.borrow_mut().children.push(Rc::clone(&child));

    // 创建循环！
    child.borrow_mut().parent = RefCell::new(Some(Rc::clone(&root)));

    println!("警告: 这个例子展示了循环引用的问题");
    println!("根节点强引用计数: {}", Rc::strong_count(&root));
    println!("子节点强引用计数: {}", Rc::strong_count(&child));

    // 即使 drop，引用计数也不会降为 0
    drop(root);
    drop(child);

    println!("内存泄漏！数据未被释放");
}

// ============================================================================ 
// 示例 4: 缓存模式
// ============================================================================ 
struct Cache {
    // 使用 Weak 允许缓存项被自动清理
    data: RefCell<Vec<Weak<String>>>, 
}

impl Cache {
    fn new() -> Cache {
        Cache {
            data: RefCell::new(vec![]),
        }
    }

    fn insert(&self, item: Rc<String>) {
        self.data.borrow_mut().push(Rc::downgrade(&item));
    }

    fn get_valid_items(&self) -> Vec<String> {
        self.data
            .borrow()
            .iter()
            .filter_map(|weak| weak.upgrade())
            .map(|rc| (*rc).clone())
            .collect()
    }
}

fn example4_cache_pattern() {
    let cache = Cache::new();

    // 添加一些数据
    let item1 = Rc::new(String::from("数据 1"));
    let item2 = Rc::new(String::from("数据 2"));

    cache.insert(item1.clone());
    cache.insert(item2.clone());

    println!("缓存中的有效项: {:?}", cache.get_valid_items());

    // 丢弃一个强引用
    drop(item1);

    println!("丢弃一个引用后: {:?}", cache.get_valid_items());
}

// ============================================================================ 
// 示例 5: Arc + Weak（线程安全）
// ============================================================================ 
fn example5_arc_weak() {
    let strong = Arc::new(42);
    let weak = Arc::downgrade(&strong);

    let handle = thread::spawn(move || {
        // 线程尝试升级弱引用
        match weak.upgrade() {
            Some(arc) => println!("线程升级成功: {}", *arc),
            None => println!("线程升级失败"),
        }
    });

    // 主线程可以继续持有强引用
    println!("主线程值: {}", *strong);

    handle.join().unwrap();
}

// ============================================================================ 
// 示例 6: 双向链表（正确的实现）
// ============================================================================ 
struct DoublyLinkedNode {
    value: i32,
    next: RefCell<Option<Rc<RefCell<DoublyLinkedNode>>>>,
    prev: RefCell<Weak<RefCell<DoublyLinkedNode>>>, 
}

impl DoublyLinkedNode {
    fn new(value: i32) -> Rc<RefCell<DoublyLinkedNode>> {
        Rc::new(RefCell::new(DoublyLinkedNode { 
            value,
            next: RefCell::new(None),
            prev: RefCell::new(Weak::new()),
        }))
    }

    fn link(a: &Rc<RefCell<DoublyLinkedNode>>, b: Rc<RefCell<DoublyLinkedNode>>) {
        // 设置 a 的下一个为 b
        *a.borrow_mut().next.borrow_mut() = Some(Rc::clone(&b));
        // 设置 b 的上一个为 a（使用弱引用）
        *b.borrow_mut().prev.borrow_mut() = Rc::downgrade(a);
    }
}

fn example6_doubly_linked_list() {
    let node1 = DoublyLinkedNode::new(1);
    let node2 = DoublyLinkedNode::new(2);
    let node3 = DoublyLinkedNode::new(3);

    DoublyLinkedNode::link(&node1, node2.clone());
    DoublyLinkedNode::link(&node2, node3.clone());

    println!("双向链表:");
    print!("正向: ");
    let mut current = Some(node1);
    while let Some(node) = current {
        print!("{} ", node.borrow().value);
        current = node.borrow().next.borrow().clone();
    }
    println!();

    // 反向遍历（使用 Weak）
    print!("反向: ");
    let mut current = Some(node3);
    while let Some(node) = current {
        print!("{} ", node.borrow().value);
        if let Some(prev) = node.borrow().prev.borrow().upgrade() {
            current = Some(prev);
        } else {
            break;
        }
    }
    println!();
}

// ============================================================================ 
// 示例 7: Weak 引用计数
// ============================================================================ 
fn example7_weak_count() {
    let strong = Rc::new(42);
    let weak1 = Rc::downgrade(&strong);
    let _weak2 = Rc::downgrade(&strong);

    println!("强引用计数: {}", Rc::strong_count(&strong));
    println!("弱引用计数: {}", Rc::weak_count(&strong));

    // 丢弃弱引用
    drop(weak1);
    println!("丢弃一个弱引用后:");
    println!("  强引用计数: {}", Rc::strong_count(&strong));
    println!("  弱引用计数: {}", Rc::weak_count(&strong));
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== Weak<T> 弱引用智能指针示例 ===\n");

    println!("示例 1: 基本 Weak 用法");
    example1_basic_weak();
    println!();

    println!("示例 2: 避免引用循环 - 树结构");
    example2_tree_without_cycles();
    println!();

    println!("示例 3: 有循环引用的树（对比）");
    example3_tree_with_cycles();
    println!();

    println!("示例 4: 缓存模式");
    example4_cache_pattern();
    println!();

    println!("示例 5: Arc + Weak（线程安全）");
    example5_arc_weak();
    println!();

    println!("示例 6: 双向链表（正确的实现）");
    example6_doubly_linked_list();
    println!();

    println!("示例 7: Weak 引用计数");
    example7_weak_count();

    println!("\n=== 总结 ===");
    println!("Weak<T> 特点:");
    println!("  - 不持有所有权");
    println!("  - 不增加强引用计数");
    println!("  - 通过 upgrade() 获取强引用");
    println!("  - 用于打破引用循环");
    println!("  - 适合缓存、观察者等场景");
    println!("  - 必须与 Rc 或 Arc 配合使用");
    println!("  - 线程安全的版本是 Arc::downgrade");
}