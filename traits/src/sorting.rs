const THRESHOLD: usize = 47;

pub fn bubble_sort(arr: &mut [i32]) {
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

pub fn selection_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        let mut min_idx = i;
        for j in i + 1..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

pub fn insertion_sort(arr: &mut [i32]) {
    insertion_sort_range(arr, 0, arr.len());
}

fn insertion_sort_range(arr: &mut [i32], start: usize, end: usize) {
    if start >= end {
        return;
    }
    for i in start + 1..end {
        let key = arr[i];
        let mut j = i;
        while j > start && arr[j - 1] > key {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = key;
    }
}

pub fn quick_sort(arr: &mut [i32]) {
    let n = arr.len();
    if n < 2 {
        return;
    }
    quick_sort_recursion(arr, 0, n);
}

pub fn std_sort(arr: &mut [i32]) {
    arr.sort_unstable();
}

fn quick_sort_recursion(arr: &mut [i32], start: usize, end: usize) {
    if end - start < THRESHOLD {
        insertion_sort_range(arr, start, end);
        return;
    }
    let pivot_idx = partition(arr, start, end);

    quick_sort_recursion(arr, start, pivot_idx);
    quick_sort_recursion(arr, pivot_idx + 1, end);
}

fn partition(arr: &mut [i32], start: usize, end: usize) -> usize {
    let pivot_index = rand::random_range(start..end);
    let pivot_value = arr[pivot_index];

    arr.swap(pivot_index, start);

    let mut l = start + 1;
    let mut r = end - 1;

    loop {
        while l <= r && arr[l] < pivot_value {
            l += 1;
        }
        while l <= r && arr[r] > pivot_value {
            r -= 1;
        }
        if l >= r {
            break;
        }
        arr.swap(l, r);
        l += 1;
        r -= 1;
    }

    arr.swap(start, r);
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort_short() {
        let mut data = vec![5, 3];
        quick_sort(&mut data);
        assert_eq!(data, vec![3, 5]);

        let mut data2 = vec![2, 1];
        quick_sort(&mut data2);
        assert_eq!(data2, vec![1, 2]);
    }

    #[test]
    fn test_all_algorithms_random() {
        use rand::Rng;
        let mut rng = rand::rng();

        for _ in 0..5 {
            let len = rng.random_range(50..100);
            let original_data: Vec<i32> = (0..len).map(|_| rng.random_range(-1000..1000)).collect();
            let mut expected = original_data.clone();
            expected.sort_unstable();

            // 测试 Quick Sort
            let mut data = original_data.clone();
            quick_sort(&mut data);
            assert_eq!(data, expected, "Quick Sort failed");

            // 测试 Bubble Sort
            let mut data = original_data.clone();
            bubble_sort(&mut data);
            assert_eq!(data, expected, "Bubble Sort failed");

            // 测试 Selection Sort
            let mut data = original_data.clone();
            selection_sort(&mut data);
            assert_eq!(data, expected, "Selection Sort failed");

            // 测试 Insertion Sort
            let mut data = original_data.clone();
            insertion_sort(&mut data);
            assert_eq!(data, expected, "Insertion Sort failed");
        }
    }
}
