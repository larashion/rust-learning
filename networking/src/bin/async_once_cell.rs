use tokio::sync::OnceCell;

static CONFIG: OnceCell<String> = OnceCell::const_new();

#[tokio::main]
async fn main() {
    println!("=== 异步 OnceCell 示例 ===");
    
    // 第一次调用，执行初始化
    let value = CONFIG
        .get_or_init(|| async {
            println!("正在初始化配置 (只运行一次)...");
            "配置值".to_string()
        })
        .await;
    println!("获取到配置: {}", value);

    // 第二次调用，直接获取值
    let value2 = CONFIG
        .get_or_init(|| async { 
            println!("这段代码不会被执行");
            "错误值".to_string() 
        })
        .await;
    println!("再次获取配置: {}", value2);
}
