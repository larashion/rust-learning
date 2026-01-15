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
use std::time::{Duration, Instant};

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                swapped = true;
                arr.swap(j, j + 1);
            }
        }
        if !swapped {
            break;
        }
    }
}

fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if i == min_idx {
            continue;
        }
        arr.swap(i, min_idx);
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    quick_sort_recursion(arr);
}

fn quick_sort_recursion(arr: &mut [i32]) {
    if arr.len() < 60 {
        insertion_sort(arr);
        return;
    }
    let pivot = partition(arr);
    let (left, right) = arr.split_at_mut(pivot + 1);
    quick_sort_recursion(left);
    quick_sort_recursion(right);
}

fn insertion_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 1..n {
        let key = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        // 也可以通过冒泡多次交换，将插入值搬到正确位置，但写入太多
        arr[j] = key;
    }
}

fn partition(arr: &mut [i32]) -> usize {
    let mut l = 0;
    let mut r = arr.len() - 1;
    // 随机选择 pivot，范围 [l, r)，即排除最后一个元素以防无限递归
    let pivot_index = rand::rng().random_range(l..r);
    let pivot_value = arr[pivot_index];

    loop {
        while arr[l] < pivot_value {
            l += 1;
        }
        while arr[r] > pivot_value {
            r -= 1;
        }
        if l >= r {
            break;
        }
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }
    r
}
// 高阶函数：接受一个闭包 F
// 我们强制要求 F 是 Fn (不可变借用)，因为这是一个纯粹的计算函数，
// 不应该允许闭包修改外部状态（比如计数器）。语义更明确。
fn calculate<F>(f: F, arr_origin: &[i32]) -> Duration
where
    F: Fn(&mut [i32]),
{
    let mut arr = arr_origin.to_vec(); // 复制一份数据，避免影响原数据
    let start = Instant::now();
    f(&mut arr);
    start.elapsed()
}
fn example_benchmark() {
    println!("--- 算法性能测试 (Algorithm Performance Test) ---");

    let len = 2000;
    let data: Vec<i32> = (0..len).rev().collect(); // 倒序数组，最坏情况

    // 1. 测试冒泡排序 (Bubble Sort)
    let time_bubble = calculate(bubble_sort, &data);

    // 2. 测试选择排序 (Selection Sort)
    let time_selection = calculate(selection_sort, &data);

    // 3. 测试插入排序 (Insertion Sort)
    let time_insertion = calculate(insertion_sort, &data);

    // 4. 测试快速排序 (Quick Sort - Hybrid)
    let time_quick = calculate(quick_sort, &data);

    // 5. 测试标准库排序
    let time_std = calculate(|arr| arr.sort(), &data);

    println!("---------------------------------------------------");
    println!("Algorithm      | Time Taken        | Ratio");
    println!("---------------------------------------------------");
    println!(
        "Bubble Sort    | {:<17?} | {:.2}x slower",
        time_bubble,
        time_bubble.as_secs_f64() / time_quick.as_secs_f64()
    );
    println!(
        "Selection Sort | {:<17?} | {:.2}x slower",
        time_selection,
        time_selection.as_secs_f64() / time_quick.as_secs_f64()
    );
    println!(
        "Insertion Sort | {:<17?} | {:.2}x slower",
        time_insertion,
        time_insertion.as_secs_f64() / time_quick.as_secs_f64()
    );
    println!("My QuickSort   | {:<17?} | 1.00x (Baseline)", time_quick);
    println!(
        "Std Library    | {:<17?} | {:.2}x faster",
        time_std,
        time_quick.as_secs_f64() / time_std.as_secs_f64()
    );
    println!("---------------------------------------------------");
}

fn main() {
    example_benchmark();
}
