use std::collections::HashMap;

/// 无序比较两个二维数组（忽略子数组顺序）
pub fn compare_2d_unsorted(result: &[Vec<i32>], expected: &[Vec<i32>]) -> bool {
    if result.len() != expected.len() {
        return false;
    }

    let mut expected_counts: HashMap<Vec<i32>, usize> = HashMap::new();
    for arr in expected {
        *expected_counts.entry(arr.clone()).or_insert(0) += 1;
    }

    for arr in result {
        match expected_counts.get_mut(arr) {
            Some(count) if *count > 0 => *count -= 1,
            _ => return false,
        }
    }

    expected_counts.values().all(|&c| c == 0)
}
