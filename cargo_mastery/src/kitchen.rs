// ============================================================================ 
// 新式模块定义 (kitchen.rs)
// ============================================================================ 
// 这是一个与文件名同名的模块。
// 对应 lib.rs 里的 `pub mod kitchen;`

// 1. 公有结构体
pub struct Breakfast {
    pub toast: String,      // 公有字段：外部可以读写
    seasonal_fruit: String, // 私有字段：外部无法直接访问（必须通过关联函数构造）
}

// 2. 也是公有枚举
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    // 构造函数：处理私有字段初始化
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

pub fn order_breakfast() {
    println!("Kitchen: Configuring breakfast...");
    let mut meal = Breakfast::summer("Rye");
    // 可以修改公有字段
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    
    // meal.seasonal_fruit = String::from("blueberries"); // ❌ 错误：私有字段
}
