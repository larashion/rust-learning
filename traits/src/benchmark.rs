use std::time::{Duration, Instant};

/// 存储测试结果的结构体
pub struct BenchResult {
    pub name: &'static str,
    pub time: Duration,
}

pub type SortFn = fn(&mut [i32]);
pub type Algo = (&'static str, SortFn);

pub fn calculate<F>(f: F, arr_origin: &[i32]) -> Duration
where
    F: Fn(&mut [i32]),
{
    let mut arr = arr_origin.to_vec(); // 复制一份数据，避免影响原数据
    let start = Instant::now();
    f(&mut arr);
    start.elapsed()
}
