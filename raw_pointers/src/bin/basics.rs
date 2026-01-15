// ============================================================================
// 裸指针 (Raw Pointers) - Unsafe Rust 的基石
// ============================================================================
//
// Rust 提供了两种裸指针：
// 1. *const T (不可变裸指针)
// 2. *mut T  (可变裸指针)
//
// 裸指针特点：
// - 允许忽略借用规则（可以同时拥有多个可变指针，或可变+不可变指针）
// - 不保证指向有效的内存
// - 允许为空 (null)
// - 没有任何自动清理（不会自动 drop）
//
// ! 重要：解引用裸指针是 unsafe 操作
//

fn main() {
    println!("=== 裸指针基础 (Raw Pointers Basics) ===\n");

    example_from_references();
    println!();

    example_from_address();
    println!();
    
    example_pointer_arithmetic();
    println!();

    println!("注意：裸指针主要用于与 C 语言交互 (FFI) 或构建底层抽象（如 Vec, Box 等）。");
    println!("在日常 Rust 编程中，应尽量避免使用。");
}

fn example_from_references() {
    println!("--- 1. 从引用创建裸指针 ---");
    
    let mut num = 5;

    // 将引用转为裸指针是安全的（不需要 unsafe块）
    // 因为这只是创建了一个地址值，并没有访问内存
    let r1 = &num as *const i32; // 不可变裸指针
    let r2 = &mut num as *mut i32; // 可变裸指针

    println!("r1 address: {:?}", r1);
    println!("r2 address: {:?}", r2);

    // 危险的操作来了：解引用
    unsafe {
        // *r1 // 读
        // *r2 = 10; // 写
        
        println!("通过 r1 读取: {}", *r1);
        *r2 = 10;
        println!("通过 r2 修改后: {}", *r2);
    }
    
    // num 被修改了
    println!("num 的值: {}", num);
}

fn example_from_address() {
    println!("--- 2. 从内存地址创建裸指针 ---");
    
    // 获取一个内存地址 (usize)
    let address = 0x012345usize;
    
    // 转换为指针
    let _r = address as *const i32;
    
    println!("创建了一个指向 0x{:x} 的指针", address);
    
    // 注意：这里绝对不能解引用！因为这个地址很可能是无效的。
    // unsafe { println!("{}", *r); } // 极大概率导致 Segmentation Fault
}

fn example_pointer_arithmetic() {
    println!("--- 3. 指针算术运算 ---");
    
    // 数组数据在内存中是连续的
    let arr = [10, 20, 30, 40, 50];
    let ptr = arr.as_ptr(); // 获取指向数组首元素的裸指针
    
    unsafe {
        println!("ptr 指向: {}", *ptr); // arr[0]
        
        // 指针偏移
        // offset(2) 意味着向后移动 2 * size_of::<i32>() 个字节
        let ptr_2 = ptr.add(2); 
        println!("ptr + 2 指向: {}", *ptr_2); // arr[2]
        
        let ptr_4 = ptr.add(4);
        println!("ptr + 4 指向: {}", *ptr_4); // arr[4]
    }
}
