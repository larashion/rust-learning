// ============================================================================
// Deref Coercion (解引用强制转换) - 隐式魔法
// ============================================================================
//
// 这是 Rust 中唯一“自动”发生的类型转换。
// 当某个类型实现了 Deref trait 时，Rust 编译器可以在函数传参、方法调用时
// 自动将 &T 转换为 &U (如果 T 实现了 Deref<Target=U>)。
//

use std::ops::Deref;

fn main() {
    println!("=== Deref Coercion (隐式转换) ===\n");

    example_vec_deref();
    println!();
    
    example_custom_smart_pointer();
}

fn example_vec_deref() {
    println!("--- 1. Vec<T> -> [T] 的魔法 ---");
    
    let v = vec![1, 2, 3];
    
    // *v 动作详解：
    // 1. 编译器发现 v 是智能指针，且实现了 Deref trait。
    // 2. 编译器将 *v 展开为 *(v.deref())。
    //    - 内部的 v.deref() 返回 &Target (即 &[i32])，这是一个普通引用。
    //    - 外部的 * 对这个普通引用进行解引用，得到 Target (即 [i32])。
    // 3. 结果是一个 [i32] (Unsized type, 动态大小类型)。
    // 4. 我们不能直接使用 Unsized 类型作为变量，必须加 & 再次变为引用 (胖指针)。
    
    println!("显式解引用再引用 (Vec -> Slice): {:?}", &*v);
    
    // 利用 Rust 的自动隐式转换 (Deref Coercion)：
    // 当我们需要 &[i32] (slice引用) 时，传入 &Vec<i32> 会自动转换
    println!("隐式转换 (Vec via coercion): {:?}", &v);
    
    // 实际应用场景：
    // 函数签名要求 &[i32]，我们可以直接传 &Vec<i32>
    print_slice(&v);
}

fn print_slice(slice: &[i32]) {
    println!("接收到了 slice，长度: {}", slice.len());
}

// --- 自定义示例 ---

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn example_custom_smart_pointer() {
    println!("--- 2. 自定义智能指针的 Deref ---");
    
    let m = MyBox::new(String::from("Rust"));
    
    // 这里发生了惊人的转换链：
    // &MyBox<String> -> &String  (通过 MyBox::deref)
    // &String        -> &str     (通过 String::deref)
    hello(&m);
    
    // 如果没有 Deref Coercion，我们需要这样写：
    // hello(&(*(*m)));
}
