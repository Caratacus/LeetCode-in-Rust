// Tests for Problem 0969: Pancake Sorting
// Java reference: src/test/java/g0901_1000/s0969_pancake_sorting/SolutionTest.java

use leetcode_in_rust::s0969::pancake_sorting::Solution;

fn is_sorted_via_pancake(arr: &[i32], flips: &[i32]) -> bool {
    let mut result = arr.to_vec();
    for &k in flips {
        let k = k as usize;
        result[..k].reverse();
    }
    result.windows(2).all(|w| w[0] <= w[1])
}

#[test]
fn test_pancake_sort() {
    let result = Solution::pancake_sort(vec![3, 2, 4, 1]);
    assert!(is_sorted_via_pancake(&[3, 2, 4, 1], &result));
}

#[test]
fn test_pancake_sort2() {
    let result = Solution::pancake_sort(vec![1, 2, 3]);
    assert!(is_sorted_via_pancake(&[1, 2, 3], &result));
}
