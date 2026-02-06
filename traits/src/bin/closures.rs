// ============================================================================
// 闭包 Trait (Fn, FnMut, FnOnce)
// ============================================================================
//
// 闭包（Closure）是可以捕获环境的匿名函数。Rust 根据闭包如何使用捕获的变量，
// 自动实现以下三个 Trait 之一（或多个）：
//
// 1. FnOnce: 消费从周围作用域捕获的变量（拿走所有权）。至少能被调用一次。
// 2. FnMut:  可变借用捕获的变量。
// 3. Fn:     不可变借用捕获的变量。

// ============================================================================
use rand::Rng;
use std::thread;

// 引入我们的库模块
use learning_traits::benchmark::{calculate, Algo, BenchResult};
use learning_traits::sorting;

fn example_benchmark() {
    println!("--- 算法性能测试 (并行 & 批量版) ---");

    let len = 20_000;
    let mut rng = rand::rng();
    let data: Vec<i32> = (0..len).map(|_| rng.random_range(0..len)).collect();

    // 定义要测试的算法列表
    let algorithms: [Algo; 5] = [
        ("Bubble Sort", sorting::bubble_sort),
        ("Selection Sort", sorting::selection_sort),
        ("Insertion Sort", sorting::insertion_sort),
        ("My QuickSort", sorting::quick_sort),
        ("Std Library", sorting::std_sort),
    ];

    // 使用循环自动并行处理
    let results = thread::scope(|s| {
        let mut handles = Vec::new();

        // 第一步：启动所有线程
        for (name, func) in algorithms {
            let data_ref = &data; // 创建一个引用
            let h = s.spawn(move || BenchResult {
                name,
                time: calculate(func, data_ref), // move 进去的是引用
            });
            handles.push(h);
        }

        // 第二步：收集所有结果
        let mut collected = Vec::new();
        for h in handles {
            collected.push(h.join().unwrap());
        }
        collected
    });

    // 找到基准时间 (QuickSort)
    let time_quick = results
        .iter()
        .find(|r| r.name == "My QuickSort")
        .map(|r| r.time)
        .unwrap();

    println!("---------------------------------------------------");
    println!("Algorithm      | Time Taken        | Ratio");
    println!("---------------------------------------------------");

    for res in results {
        let ratio = match res.name {
            "My QuickSort" => "1.00x (Baseline)".to_string(),
            _ if res.time < time_quick => {
                format!(
                    "{:.2}x faster",
                    time_quick.as_secs_f64() / res.time.as_secs_f64()
                )
            }
            _ => {
                format!(
                    "{:.2}x slower",
                    res.time.as_secs_f64() / time_quick.as_secs_f64()
                )
            }
        };

        println!("{:<15} | {:<17?} | {}", res.name, res.time, ratio);
    }
    println!("---------------------------------------------------");
}

fn main() {
    example_benchmark();
}
