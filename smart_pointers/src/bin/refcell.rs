// ============================================================================
// RefCell<T> - 内部可变性模式
// ============================================================================
//
// RefCell<T> 提供内部可变性（Interior Mutability）。
//
// 主要特点：
// 1. 在运行时（而非编译时）检查借用规则
// 2. 允许通过不可变引用修改数据
// 3. 违反借用规则会 panic（在 Debug 模式下）
// 4. 单线程专用（不是 Send + Sync）
// 5. 常与 Rc 组合使用

use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// 示例 1: 基本用法 - 内部可变性
// ============================================================================
fn example1_basic_refcell() {
    // RefCell 允许通过不可变引用修改内部数据
    let data = RefCell::new(42);

    // 借用不可变引用
    {
        let borrowed = data.borrow();
        println!("不可变借用: {}", *borrowed);
    }

    // 借用可变引用
    {
        let mut borrowed = data.borrow_mut();
        *borrowed = 100;
        println!("修改后: {}", *borrowed);
    }

    println!("最终值: {}", *data.borrow());
}

// ============================================================================
// 示例 2: 运行时借用检查
// ============================================================================
fn example2_runtime_borrow_check() {
    let data = RefCell::new(42);

    // 创建两个不可变借用 - 这是允许的
    let borrow1 = data.borrow();
    let borrow2 = data.borrow();
    println!("两个不可变借用: {}, {}", *borrow1, *borrow2);

    // drop(borrow1); // 取消注释可以避免 panic
    // drop(borrow2);

    // 尝试在不可变借用存在时创建可变借用 - 会 panic！
    // let mut borrow_mut = data.borrow_mut(); // panic!
    println!("警告: 如果尝试在不可变借用时获取可变借用，会 panic");
}

// ============================================================================
// 示例 3: 与 Rc 组合 - 多所有者可变访问
// ============================================================================
struct MockMessenger {
    messages: RefCell<Vec<String>>,
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            messages: RefCell::new(vec![]),
        }
    }

    fn send(&self, message: &str) {
        // 即使 self 是不可变引用，也能修改 messages
        self.messages.borrow_mut().push(String::from(message));
    }

    fn get_messages(&self) -> Vec<String> {
        self.messages.borrow().clone()
    }
}

fn example3_rc_refcell_pattern() {
    let messenger = Rc::new(MockMessenger::new());

    // 多个所有者共享同一个 messenger
    let messenger1 = Rc::clone(&messenger);
    let messenger2 = Rc::clone(&messenger);

    // 所有者都可以修改消息
    messenger1.send("消息 1");
    messenger2.send("消息 2");
    messenger.send("消息 3");

    println!("所有消息: {:?}", messenger.get_messages());
}

// ============================================================================
// 示例 4: try_borrow 和 try_borrow_mut
// ============================================================================
fn example4_try_borrow() {
    let data = RefCell::new(42);

    // 不可变借用
    let _borrow = data.borrow();

    // 尝试获取可变借用
    // 注意：如果有其他借用存在，这里会返回 Err，而不是 panic
    let result = data.try_borrow_mut();
    match result {
        Ok(mut val) => {
            *val += 1;
            println!("修改后的值: {}", *val);
        }
        Err(e) => {
            println!("无法获取可变借用: {}", e);
        }
    }
}

// ============================================================================
// 示例 5: 模拟可变句柄
// ============================================================================
struct Database {
    data: RefCell<Vec<i32>>,
}

impl Database {
    fn new() -> Database {
        Database {
            data: RefCell::new(vec![]),
        }
    }

    fn add(&self, item: i32) {
        // 即使 add 方法接收不可变 self，也能修改内部数据
        self.data.borrow_mut().push(item);
    }

    fn get_all(&self) -> Vec<i32> {
        self.data.borrow().clone()
    }

    fn clear(&self) {
        self.data.borrow_mut().clear();
    }
}

fn example5_mut_handle() {
    let db = Database::new();

    db.add(1);
    db.add(2);
    db.add(3);

    println!("数据库内容: {:?}", db.get_all());
    db.clear();
    println!("清空后: {:?}", db.get_all());
}

// ============================================================================
// 示例 6: 嵌套的 RefCell
// ============================================================================
struct Inner {
    value: RefCell<i32>,
}

struct Outer {
    inner: RefCell<Inner>,
}

fn example6_nested_refcell() {
    let outer = Outer {
        inner: RefCell::new(Inner {
            value: RefCell::new(10),
        }),
    };

    // 嵌套访问和修改
    {
        let inner = outer.inner.borrow();
        let value = inner.value.borrow();
        println!("嵌套值: {}", *value);
    }

    {
        let inner = outer.inner.borrow_mut();
        *inner.value.borrow_mut() = 20;
    }

    println!("修改后嵌套值: {}", *outer.inner.borrow().value.borrow());
}

// ============================================================================
// 示例 7: RefCell 在 trait 实现中的使用
// ============================================================================
trait Storage {
    fn store(&self, item: i32);
    fn retrieve(&self) -> Option<i32>;
}

struct SimpleStorage {
    item: RefCell<Option<i32>>,
}

impl Storage for SimpleStorage {
    fn store(&self, item: i32) {
        *self.item.borrow_mut() = Some(item);
    }

    fn retrieve(&self) -> Option<i32> {
        *self.item.borrow()
    }
}

fn example7_trait_impl() {
    let storage = SimpleStorage {
        item: RefCell::new(None),
    };

    storage.store(42);
    println!("存储的值: {:?}", storage.retrieve());
}

// ============================================================================
// 示例 8: 避免内存泄漏
// ============================================================================
fn example8_leak_prevention() {
    // RefCell 的借用会在离开作用域时自动释放
    let data = RefCell::new(42);

    {
        let _borrow = data.borrow();
        // borrow 在这里自动释放
    }

    // 现在可以获取可变借用
    let mut borrow_mut = data.borrow_mut();
    *borrow_mut = 100;

    println!("成功修改: {}", *borrow_mut);
}

// ============================================================================
// 主函数
// ============================================================================
fn main() {
    println!("=== RefCell<T> 内部可变性示例 ===\n");

    println!("示例 1: 基本用法 - 内部可变性");
    example1_basic_refcell();
    println!();

    println!("示例 2: 运行时借用检查");
    example2_runtime_borrow_check();
    println!();

    println!("示例 3: 与 Rc 组合");
    example3_rc_refcell_pattern();
    println!();

    println!("示例 4: try_borrow 和 try_borrow_mut");
    example4_try_borrow();
    println!();

    println!("示例 5: 模拟可变句柄");
    example5_mut_handle();
    println!();

    println!("示例 6: 嵌套的 RefCell");
    example6_nested_refcell();
    println!();

    println!("示例 7: RefCell 在 trait 实现中的使用");
    example7_trait_impl();
    println!();

    println!("示例 8: 避免内存泄漏");
    example8_leak_prevention();

    println!("\n=== 总结 ===");
    println!("RefCell<T> 特点:");
    println!("  - 运行时借用检查（vs 编译时）");
    println!("  - 允许内部可变性");
    println!("  - 违反借用规则会 panic");
    println!("  - 单线程专用");
    println!("  - 常与 Rc 组合使用");
    println!("  - 谨慎使用：运行时错误更难调试");
    println!("  - borrow()/borrow_mut() 返回 Ref/RefMut 智能指针");
}
