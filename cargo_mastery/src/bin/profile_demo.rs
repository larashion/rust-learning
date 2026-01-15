// ============================================================================ 
// Cargo Profiles 演示: Debug vs Release
// ============================================================================ 
// 
// ============================================================================ 
// Cargo Profiles 深层解析: Debug vs Release
// ============================================================================ 
// 
// 知识点: Profile (配置)
// Cargo 有两个主要的预设配置：
//
// 1. dev (使用 `cargo build` 或 `cargo run` 时)
//    - 目标: 开发体验优先，编译速度快，但这生成的二进制跑得慢。
//    - `opt-level = 0`: 几乎不优化，方便调试器单步执行。
//    - `debug-assertions = true`: 开启溢出检查 (integer overflow checks) 等安全网。
//    - `overflow-checks = true`: 例如 `255u8 + 1` 会 panic 而不是静默回绕。
//
// 2. release (使用 `cargo build --release` 时)
//    - 目标: 运行时性能优先，生成的二进制体积小，但在编译时会花更多时间。
//    - `opt-level = 3`: 开启最高级别优化 (循环展开、内联、向量化)。
//    - `debug-assertions = false`: 关闭昂贵的运行时检查。
//    - `lto = false` (默认): 可以在 Cargo.toml 里开启 Link Time Optimization 进一步压榨性能。
//
// 关键结论:
// **哪怕只是随便跑跑测试，只要涉及"耗时"或"基准测试"，必须加 --release！**
// 否则你测的不是代码的性能，而是调试器的开销。
//
// 预期结果: 本程序的 Release 模式通常比 Debug 模式快 10-100 倍！

use std::time::Instant;

fn main() {
    println!("=== Performance Demo: Matrix Multiplication ===");
    println!("Mode: {}", if cfg!(debug_assertions) { "DEBUG (Slow)" } else { "RELEASE (Fast)" });

    let size = 400; // 400x400 矩阵乘法
    
    // 生成矩阵
    let matrix_a = vec![vec![1.5; size]; size];
    let matrix_b = vec![vec![2.5; size]; size];
    let mut result = vec![vec![0.0; size]; size];

    let start = Instant::now();

    // 朴素矩阵乘法 O(N^3)
    // 编译器极难优化这个嵌套循环，除非开启 Release
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += matrix_a[i][k] * matrix_b[k][j];
            }
        }
    }

    let duration = start.elapsed();
    
    // 验证结果 (防止被过度优化掉)
    println!("Result [0][0]: {}", result[0][0]);
    println!("Time taken: {:.2?}", duration);

    if cfg!(debug_assertions) {
        println!("\n[TIP] Try running with `cargo run -p cargo-mastery --release --bin profile_demo` to see the magic!");
    } else {
        println!("\n[SUCCESS] Running at full speed!");
    }
}
