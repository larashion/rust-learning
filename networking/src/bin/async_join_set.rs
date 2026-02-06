use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("=== Tokio JoinSet 示例 ===");
    
    let mut set = JoinSet::new();

    // 1. 批量启动不同耗时的任务
    for i in 1..=5 {
        set.spawn(async move {
            // 模拟不同任务耗时不同，i=1 最慢，i=5 最快
            let delay = 6 - i; 
            sleep(Duration::from_millis(delay * 200)).await;
            
            if i == 3 {
                // 模拟一个任务失败
                return Err(format!("任务 {} 发生了错误", i));
            }
            
            Ok(format!("任务 {} 完成 (耗时 {}ms)", i, delay * 200))
        });
    }

    // 2. 按完成顺序（谁先跑完谁先出来）处理结果
    // 这比 join_all 灵活得多，因为它不需要等待所有任务都结束才开始处理第一个结果
    println!("等待任务结果 (按完成顺序):");
    while let Some(res) = set.join_next().await {
        match res {
            Ok(Ok(msg)) => println!("成功: {}", msg),
            Ok(Err(e)) => eprintln!("业务逻辑错误: {}", e),
            Err(e) => eprintln!("任务 Join 错误 (可能是 Panic): {}", e),
        }
    }

    // 3. 演示 JoinSet 的“动态性”
    println!("
--- 演示动态添加任务 ---");
    let mut dynamic_set = JoinSet::new();
    
    dynamic_set.spawn(async { 
        sleep(Duration::from_millis(100)).await;
        "动态任务 1"
    });

    if let Some(res) = dynamic_set.join_next().await {
        println!("第一个任务完成: {:?}", res?);
        
        // 在第一个任务完成后，我们又加了一个任务
        dynamic_set.spawn(async { "动态任务 2" });
    }

    while let Some(res) = dynamic_set.join_next().await {
        println!("处理剩余任务: {:?}", res?);
    }

    println!("
所有示例执行完毕。");
    Ok(())
}
