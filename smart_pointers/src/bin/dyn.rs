#![allow(unused)]
// ============================================================================ 
// 別分发 (Dynamic Dispatch) - dyn Trait 对象
// ============================================================================ 
//
// `dyn` 关键字用于创建 Trait 对象，实现动态分发。
//
// 主要特点：
// 1. 运行时确定具体类型
// 2. 通过 vtable（虚把表）调用方法
// 3. 动态大小类型（DST）
// 4. 必须通过引用或智能指针使用
// 5. 与类型（静态分发）相对


// ============================================================================ 
// 示例 1: 基本概念 - 静态分发 vs 动态分发
// ============================================================================ 
trait Animal {
    fn make_sound(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn make_sound(&self) {
        println!("汹汹！");
    }
}

impl Animal for Cat {
    fn make_sound(&self) {
        println!("啰啰！");
    }
}

fn example1_basic_concept() {
    println!("=== 静态分发（类型）===");
    let dog = Dog;
    let cat = Cat;

    // 编译时确定类型，为每个类型生成独立代码
    make_sound_static(&dog);
    make_sound_static(&cat);

    println!("\n=== 动态分发（Trait 对象）===");
    let dog = Box::new(Dog);
    let cat = Box::new(Cat);

    // 运行时确定类型，通过 vtable 调用
    let animals: Vec<Box<dyn Animal>> = vec![dog, cat];

    for animal in animals {
        animal.make_sound(); // 动态分发
    }
}

// 静态分发：类型
fn make_sound_static<T: Animal>(animal: &T) {
    animal.make_sound();
}

// ============================================================================ 
// 示例 2: Trait 对象的形式
// ============================================================================ 
fn example2_trait_object_forms() {
    // 1. &dyn Trait（引用）
    let dog = Dog;
    let animal: &dyn Animal = &dog;
    animal.make_sound();

    // 2. &mut dyn Trait（可变引用）
    let mut cat = Cat;
    let animal: &mut dyn Animal = &mut cat;
    animal.make_sound();

    // 3. Box<dyn Trait>（堆分配）
    let animal: Box<dyn Animal> = Box::new(Dog);
    animal.make_sound();

    // 4. Rc<dyn Trait>（共享所有权）
    use std::rc::Rc;
    let animal: Rc<dyn Animal> = Rc::new(Cat);
    animal.make_sound();

    // 5. Arc<dyn Trait>（繁繁安全共享）
    use std::sync::Arc;
    let animal: Arc<dyn Animal> = Arc::new(Dog);
    animal.make_sound();

    // 注意：不能直接创建 Trait 对象，必须通过指针
    // let animal: dyn Animal = Dog; // 编译错误！
}

// ============================================================================ 
// 示例 3: 动态分发的性能
// ============================================================================ 
use std::time::Instant;

fn example3_performance() {
    const ITERATIONS: usize = 10_000_000;

    let dog = Dog;
    let cat = Cat;

    // 静态分发
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        make_sound_static(&dog);
        make_sound_static(&cat);
    }
    let static_time = start.elapsed();

    // 动态分发
    let animals: Vec<&dyn Animal> = vec
![&dog, &cat];
    let start = Instant::now();
    for _ in 0..ITERATIONS {
        for animal in &animals {
            animal.make_sound();
        }
    }
    let dynamic_time = start.elapsed();

    println!("静态分发耗时: {:?}", static_time);
    println!("动态分发耗时: {:?}", dynamic_time);
    println!("性能差异: 静态分发通常更快（编译时优化）");
}

// ============================================================================ 
// 示例 4: 对象安全（Object Safety）
// ============================================================================ 
fn example4_object_safety() {
    // 对象安全的 trait 可以创建 Trait 对象

    // ✅ 对象安全：方法只有 Self 引用
    trait Printable {
        fn print(&self);
    }

    // ✅ 对象安全：没有类型参数
    trait Display {
        fn display(&self);
    }

    // ❌ 非对象安全：方法返回 Self
    trait Cloneable {
        fn clone(&self) -> Self; // 返回 Self，不能创建 Trait 对象
    }

    // ❌ 非对象安全：方法有类型参数
    trait GenericMethod {
        fn process<T>(&self, item: T); // 类型参数，不能创建 Trait 对象
    }

    // ✅ 对象安全：使用关联类型
    trait Associated {
        type Output;
        fn get(&self) -> Self::Output;
    }

    struct MyStruct;
    impl Associated for MyStruct {
        type Output = i32;
        fn get(&self) -> Self::Output {
            42
        }
    }

    let obj: Box<dyn Associated<Output = i32>> = Box::new(MyStruct);
    println!("关联类型: {}", obj.get());
}

// ============================================================================ 
// 示例 5: Trait 对象作为返回值
// ============================================================================ 
fn example5_return_trait_object() {
    // 返回 Trait 对象
    let animal = create_animal("dog");
    animal.make_sound();

    let animal = create_animal("cat");
    animal.make_sound();
}

fn create_animal(type_name: &str) -> Box<dyn Animal> {
    match type_name {
        "dog" => Box::new(Dog),
        "cat" => Box::new(Cat),
        _ => panic!("未知动物类型"),
    }
}

// ============================================================================ 
// 示例 6: Trait 对象作为函数参数
// ============================================================================ 
fn example6_trait_object_parameter() {
    let dog = Dog;
    let cat = Cat;

    // 接受 Trait 对象
    process_animal(&dog);
    process_animal(&cat);

    // 使用 Trait 对象集合
    let animals: Vec<&dyn Animal> = vec
![&dog, &cat];
    process_animals(&animals);
}

fn process_animal(animal: &dyn Animal) {
    println!("处理动物...");
    animal.make_sound();
}

fn process_animals(animals: &[&dyn Animal]) {
    println!("处理 {} 只动物", animals.len());
    for animal in animals {
        animal.make_sound();
    }
}

// ============================================================================ 
// 示例 7: 组合多个 Trait
// ============================================================================ 
trait Display {
    fn display(&self) -> String;
}

impl Display for Dog {
    fn display(&self) -> String {
        "Dog".to_string()
    }
}

impl Display for Cat {
    fn display(&self) -> String {
        "Cat".to_string()
    }
}

// 定义一个拼捷 Trait
trait AnimalDisplay: Animal + Display {}
impl<T: Animal + Display> AnimalDisplay for T {}

fn example7_combine_traits() {
    let dog = Dog;
    let cat = Cat;

    // 组合多个 Trait： dyn Animal + Display
    // 使用 dyn AnimalDisplay 代替 dyn Animal + Display
    let animals: Vec<Box<dyn AnimalDisplay>> = vec![
        Box::new(dog),
        Box::new(cat),
    ];

    for animal in animals {
        animal.make_sound();
        println!("  类型: {}", animal.display());
    }
}

// ============================================================================ 
// 示例 8: Trait 对象的内存布局
// ============================================================================ 
use std::mem;

fn example8_memory_layout() {
    // Trait 对象由两部分组成：
    // 1. 数据指针（指向实际数据）
    // 2. vtable 指针（指向虚场表）

    let dog = Dog;
    let _trait_obj: &dyn Animal = &dog;

    println!("Trait 对象大小:");
    println!("  &dyn Animal: {} 字节", mem::size_of::<&dyn Animal>());
    println!("  Box<dyn Animal>: {} 字节", mem::size_of::<Box<dyn Animal>>());
    println!("  Dog: {} 字节", mem::size_of::<Dog>());

    // 在 64 位 系统上：
    // &dyn Animal = 16 字节（8 字节数据指针 + 8 字节 vtable 指针）
    // Box<dyn Animal> = 16 字节（同上）
}

// ============================================================================ 
// 示例 9: Trait 对象的局限性
// ============================================================================ 
fn example9_limitations() {
    println!("Trait 对象的局限性:");
    println!("  1. 不能调用关联函数（没有 self）");
    println!("  2. 不能调用返回 Self 的方法");
    println!("  3. 不能调用有类型参数的方法");
    println!("  4. 不能自动派生 trait");
    println!("  5. 性能略低于静态分发");
}

// ============================================================================ 
// 示例 10: 实际应用 - 插件系统
// ============================================================================ 
trait Plugin {
    fn name(&self) -> &str;
    fn execute(&self);
}

struct PluginA;
struct PluginB;

impl Plugin for PluginA {
    fn name(&self) -> &str {
        "Plugin A"
    }

    fn execute(&self) {
        println!("{} 执行中...", self.name());
    }
}

impl Plugin for PluginB {
    fn name(&self) -> &str {
        "Plugin B"
    }

    fn execute(&self) {
        println!("{} 执行中...", self.name());
    }
}

struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    fn new() -> Self {
        PluginManager {
            plugins: Vec::new(),
        }
    }

    fn register(&mut self, plugin: Box<dyn Plugin>) {
        println!("注册插件: {}", plugin.name());
        self.plugins.push(plugin);
    }

    fn execute_all(&self) {
        println!("执行所有插件:");
        for plugin in &self.plugins {
            plugin.execute();
        }
    }
}

fn example10_plugin_system() {
    let mut manager = PluginManager::new();

    // 注册不同类型的插件
    manager.register(Box::new(PluginA));
    manager.register(Box::new(PluginB));

    // 执行所有插件
    manager.execute_all();
}

// ============================================================================ 
// 示例 11: 实际应用 - 策略模式
// ============================================================================ 
trait PaymentStrategy {
    fn pay(&self, amount: f64) -> Result<String, String>;
}

struct CreditCardPayment;
struct PayPalPayment;

impl PaymentStrategy for CreditCardPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        Ok(format!("xinyongka支付 ${:.2}", amount))
    }
}

impl PaymentStrategy for PayPalPayment {
    fn pay(&self, amount: f64) -> Result<String, String> {
        Ok(format!("PayPal 支付 ${:.2}", amount))
    }
}

struct ShoppingCart {
    payment_strategy: Box<dyn PaymentStrategy>,
}

impl ShoppingCart {
    fn new(strategy: Box<dyn PaymentStrategy>) -> Self {
        ShoppingCart {
            payment_strategy: strategy,
        }
    }

    fn set_payment_strategy(&mut self, strategy: Box<dyn PaymentStrategy>) {
        self.payment_strategy = strategy;
    }

    fn checkout(&self, amount: f64) {
        match self.payment_strategy.pay(amount) {
            Ok(message) => println!("{}", message),
            Err(e) => eprintln!("支付失败: {}", e),
        }
    }
}

fn example11_strategy_pattern() {
    let mut cart = ShoppingCart::new(Box::new(CreditCardPayment));
    cart.checkout(100.0);

    // 切换支付方式
    cart.set_payment_strategy(Box::new(PayPalPayment));
    cart.checkout(50.0);
}

// ============================================================================ 
// 示例 12: 实际应用 - 回调
// ============================================================================ 
trait Callback {
    fn on_event(&self, event: &str);
}

struct LogCallback;
struct AlertCallback;

impl Callback for LogCallback {
    fn on_event(&self, event: &str) {
        println!("[LOG] 事件: {}", event);
    }
}

impl Callback for AlertCallback {
    fn on_event(&self, event: &str) {
        println!("[ALERT] 事件: {} (需要关注)", event);
    }
}

struct EventBus {
    callbacks: Vec<Box<dyn Callback>>,
}

impl EventBus {
    fn new() -> Self {
        EventBus {
            callbacks: Vec::new(),
        }
    }

    fn register(&mut self, callback: Box<dyn Callback>) {
        self.callbacks.push(callback);
    }

    fn trigger(&self, event: &str) {
        println!("触发事件: {}", event);
        for callback in &self.callbacks {
            callback.on_event(event);
        }
    }
}

fn example12_callback() {
    let mut bus = EventBus::new();

    bus.register(Box::new(LogCallback));
    bus.register(Box::new(AlertCallback));

    bus.trigger("系统启动");
    bus.trigger("错误发生");
}

// ============================================================================ 
// 示例 13: 动态分发 vs 静态分发的选择
// ============================================================================ 
fn example13_when_to_use() {
    println!("使用动态分发 (dyn Trait) 的场景:");
    println!("  1. 运行时才能确定类型");
    println!("  2. 需要存储不同类型的集合");
    println!("  3. 减少 二进制囊胀");
    println!("  4. 实现回调、插件系统");
    println!("  5. API 边界（动态库）");

    println!("\n使用静态分发 (类型) 的场景:");
    println!("  1. 类型在编译时已知");
    println!("  2. 追求最高性能");
    println!("  3. 需要内联优化");
    println!("  4. 代码可读性更重要");
    println!("  5. 避免堆分配");
}

// ============================================================================ 
// 示例 14: 向下转换（Downcasting）
// ============================================================================ 
use std::any::Any;

trait Drawable: Any {
    fn draw(&self);
    fn as_any(&self) -> &dyn Any;
}

struct Circle {
    radius: f64,
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制圆形，半径: {}", self.radius);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制矩形，宽: {}, 高: {}", self.width, self.height);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn example14_downcasting() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];

    for shape in shapes {
        shape.draw();

        // 向下转换
        if let Some(circle) = shape.as_any().downcast_ref::<Circle>() {
            println!("  是圆圈，面积: {}", std::f64::consts::PI * circle.radius * circle.radius);
        } else if let Some(rectangle) = shape.as_any().downcast_ref::<Rectangle>() {
            println!("  是矩形，静积: {}", rectangle.width * rectangle.height);
        }
    }
}

// ============================================================================ 
// 示例 15: Trait 对象的相等性比较
// ============================================================================ 
fn example15_equality() {
    // Trait 对象不能直接比较，即使 trait 继承了 Eq

    // 但可以通过 Any 特征
    trait Shape {
        fn area(&self) -> f64;
    }

    struct Square {
        side: f64,
    }

    impl Shape for Square {
        fn area(&self) -> f64 {
            self.side * self.side
        }
    }

    let shapes: Vec<Box<dyn Shape>> = vec
![ 
        Box::new(Square { side: 5.0 }),
        Box::new(Square { side: 5.0 }),
    ];

    // 比较面积
    if shapes[0].area() == shapes[1].area() {
        println!("两个形状面积相同");
    }
}

// ============================================================================ 
// 示例 16: Trait 对象的克隆
// ============================================================================ 
trait Cloneable {
    fn clone_box(&self) -> Box<dyn Cloneable>;
}

#[derive(Clone)]
struct Data {
    value: i32,
}

impl Cloneable for Data {
    fn clone_box(&self) -> Box<dyn Cloneable> {
        Box::new(self.clone())
    }
}

fn example16_clone_trait_object() {
    let original: Box<dyn Cloneable> = Box::new(Data { value: 42 });
    let _cloned = original.clone_box();

    println!("Trait 对象克隆完成");
}

// ============================================================================ 
// 示例 17: Trait 对象的迷代器
// ============================================================================ 
trait Processor {
    fn process(&self, input: &str) -> String;
}

struct UppercaseProcessor;
struct LowercaseProcessor;

impl Processor for UppercaseProcessor {
    fn process(&self, input: &str) -> String {
        input.to_uppercase()
    }
}

impl Processor for LowercaseProcessor {
    fn process(&self, input: &str) -> String {
        input.to_lowercase()
    }
}

fn example17_trait_object_iterators() {
    let processors: Vec<Box<dyn Processor>> = vec
![ 
        Box::new(UppercaseProcessor),
        Box::new(LowercaseProcessor),
    ];

    let text = "Hello World";

    println!("原始文本: {}", text);
    for processor in processors {
        let result = processor.process(text);
        println!("处理结果: {}", result);
    }
}

// ============================================================================ 
// 示例 18: Trait 对象的错误处理
// ============================================================================ 
trait ErrorHandler {
    fn handle(&self, error: &str);
}

struct LogErrorHandler;
struct EmailErrorHandler;

impl ErrorHandler for LogErrorHandler {
    fn handle(&self, error: &str) {
        println!("[LOG] 错误: {}", error);
    }
}

impl ErrorHandler for EmailErrorHandler {
    fn handle(&self, error: &str) {
        println!("[EMAIL] 发送错误邮件: {}", error);
    }
}

struct Application {
    error_handler: Box<dyn ErrorHandler>,
}

impl Application {
    fn new(handler: Box<dyn ErrorHandler>) -> Self {
        Application {
            error_handler: handler,
        }
    }

    fn run(&self) {
        // 模拟错误
        self.error_handler.handle("应用发生错误");
    }
}

fn example18_error_handling() {
    let app = Application::new(Box::new(LogErrorHandler));
    app.run();

    let app = Application::new(Box::new(EmailErrorHandler));
    app.run();
}

// ============================================================================ 
// 示例 19: Trait 对象的序列化（需要额外 trait）
// ============================================================================ 
use serde::{Serialize, Deserialize};

trait Serializable {
    fn serialize(&self) -> String;
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
}

impl Serializable for Person {
    fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap_or("序列化失败".to_string())
    }
}

fn example19_serialization() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let serializable: Box<dyn Serializable> = Box::new(person);
    let json = serializable.serialize();

    println!("序列化结果: {}", json);
}

// ============================================================================ 
// 示例 20: Trait 对象的 Send 和 Sync
// ============================================================================ 
fn example20_send_sync() {
    // Trait 对象可以是 Send 和 Sync，如果 trait 佰束满足
    let dog = Dog;
    let _trait_obj: Box<dyn Animal> = Box::new(dog);

    // 可以跨线程传递（如果 Animal: Send ）
    // let handle = thread::spawn(move || {
    //     trait_obj.make_sound();
    // });

    println!("Trait 对象的线程安全性取决于 trait 佰束条件");
}

// ============================================================================ 
// 主函数
// ============================================================================ 
fn main() {
    println!("=== Rust 动态分发 (dyn Trait) 示例 ===\n");

    println!("示例 1: 基本概念 - 静态分发 vs 动态分发");
    example1_basic_concept();
    println!();

    println!("示例 2: Trait 对象的形式");
    example2_trait_object_forms();
    println!();

    println!("示例 3: 动态分发的性能");
    example3_performance();
    println!();

    println!("示例 4: 对象安全（Object Safety）");
    example4_object_safety();
    println!();

    println!("示例 5: Trait 对象作为返回值");
    example5_return_trait_object();
    println!();

    println!("示例 6: Trait 对象作为函数参数");
    example6_trait_object_parameter();
    println!();

    println!("示例 7: 组合多个 Trait");
    example7_combine_traits();
    println!();

    println!("示例 8: Trait 对象的内存布局");
    example8_memory_layout();
    println!();

    println!("示例 9: Trait 对象的局限性");
    example9_limitations();
    println!();

    println!("示例 10: 实际应用 - 插件系统");
    example10_plugin_system();
    println!();

    println!("示例 11: 实际应用 - 策略模式");
    example11_strategy_pattern();
    println!();

    println!("示例 12: 实际应用 - 回调");
    example12_callback();
    println!();

    println!("示例 13: 动态分发 vs 静态分发的选择");
    example13_when_to_use();
    println!();

    println!("示例 14: 向下转换（Downcasting）");
    example14_downcasting();
    println!();

    println!("示例 15: Trait 对象的相等性比较");
    example15_equality();
    println!();

    println!("示例 16: Trait 对象的克隆");
    example16_clone_trait_object();
    println!();

    println!("示例 17: Trait 对象的迷代器");
    example17_trait_object_iterators();
    println!();

    println!("示例 18: Trait 对象的错误处理");
    example18_error_handling();
    println!();

    println!("示例 19: Trait 对象的序列化");
    example19_serialization();
    println!();

    println!("示例 20: Trait 对象的 Send 和 Sync");
    example20_send_sync();

    println!("\n=== 总结 ===");
    println!("dyn Trait 特点:");
    println!("  - 动态分发（运行时确定类型）");
    println!("  - 通过 vtable 调用方法");
    println!("  - 必须通过指针使用（&, Box, Rc, Arc）");
    println!("  - 动态大小类型（DST）");
    println!("  - 需要对象安全（Object Safety）");
    println!("\n内存布局:");
    println!("  - 数据指针");
    println!("  - vtable 指针");
    println!("  - 总大小: 2 * 指针大小（64位系统上 16 字节）");
    println!("\n使用场景:");
    println!("  - 插件系统");
    println!("  - 策略模式");
    println!("  - 回调");
    println!("  - 运行时类型集合");
    println!("  - 减少二进制去胀");
    println!("\n性能:");
    println!("  - 略低于静态分发（vtable 查找）");
    println!("  - 但更灵活，避免代码有带有的膨胀");
    println!("  - 堆分配开销（Box）");
    println!("\n对象安全规则:");
    println!("  - 方法不能返回 Self");
    println!("  - 方法不能有类型参数");
    println!("  - 没有 Self: Sized 约束");
}