// ============================================================================
// Lifetimes - 基础概念
// ============================================================================
//
// 生命周期（Lifetimes）是 Rust 借用检查器（Borrow Checker）用于确保所有借用都是有效的机制。
//
// 核心原则：
// 1. 每一个引用都有其生命周期（即它保持有效的作用域）。
// 2. 大多数时候生命周期是隐式的（被推导出来的）。
// 3. 当生命周期以不同的方式相互关联时，我们需要手动标注（Annotation）。
//
// ⚠️ 重要观念：生命周期标注 不会 改变引用的实际存活时间，它只是向编译器描述多个引用之间的关系，
// 以便编译器验证代码安全性。

// ============================================================================
// 示例 1: 为什么需要生命周期？（悬垂引用）
// ============================================================================
fn example1_dangling_reference() {
    println!("--- 示例 1: 悬垂引用 (Dangling Reference) ---");

    // let r;                // ---------+-- r 的生命周期
    // {                     //          |
    //     let x = 5;        // -+-- x   |
    //     r = &x;           //  |       |
    // }                     // -+       |
    // println!("r: {}", r); //          |
    // --------+

    // 上面的代码会编译失败：`x` does not live long enough.
    // 因为 r 引用了 x，但 x 在花括号结束时就被 drop 了，而 r 还在使用。
    println!("(此示例为编译错误演示，已注释)");
}

// ============================================================================
// 示例 2: 函数中的泛型生命周期
// ============================================================================
// 场景：我们要写一个函数，返回两个字符串切片中较长的一个。
//
// 错误写法：
// fn longest(x: &str, y: &str) -> &str { ... }
// 编译器报错：它不知道返回的 &str 到底是来自 x 还是 y，
// 因此也不知道返回值能活多久。

// 正确写法：使用生命周期标注 <'a>
// 读作：“x 和 y 至少都活得跟 'a 一样长，而返回值也至少活得跟 'a 一样长”
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
#[allow(dead_code)]
fn longest_precise<'a, 'b, 'out>(x: &'a str, y: &'b str) -> &'out str
where
    'a: 'out,
    'b: 'out,
{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
fn example2_function_lifetimes() {
    println!("\n--- 示例 2: 函数签名中的生命周期 ---");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: {}", result);

    // 复杂情况：不同的作用域
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        // result 的生命周期取 string1 和 string2 中 *较短* 的那个
        let result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串是 (内部作用域): {}", result);
    }
}

// ============================================================================
// 示例 3: 生命周期标注的误区
// ============================================================================
// 标注只是为了通过检查，不能“延长”寿命。

// 下面的代码会报错，因为 result 的生命周期被标注为和 string2 一样长，
// 但实际使用时，result 试图活得比 string2 更久。
/*
fn example3_lifetime_mismatch() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    } // string2 在这里销毁，result 指向的引用随之失效

    println!("Result: {}", result); // ❌ 错误：result 此时是悬垂引用
}
*/

// ============================================================================
// 示例 4: 返回局部变量的引用（永远是错的）
// ============================================================================
// fn invalid_return<'a>() -> &'a str {
//     let s = String::from("I am local");
//     &s // ❌ 错误：s 离开函数就会被 drop，不能返回它的引用
// }
//
// ✅ 解决方法：直接返回所有权 (String)，而不是引用 (&str)
fn valid_return() -> String {
    String::from("I am local but I move out")
}

fn main() {
    example1_dangling_reference();
    example2_function_lifetimes();

    println!("\n--- 示例 4: 返回所有权 ---");
    println!("{}", valid_return());
}
