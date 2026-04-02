// Tests for Problem 0040: Combination Sum II
// Java reference: src/test/java/g0001_0100/s0040_combination_sum_ii/SolutionTest.java

use leetcode_in_rust::s0040::combination_sum_ii::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_combination_sum2() {
    let result = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
    let expected = vec![vec![1, 1, 6], vec![1, 2, 5], vec![1, 7], vec![2, 6]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_combination_sum22() {
    let result = Solution::combination_sum2(vec![2, 5, 2, 1, 2], 5);
    let expected = vec![vec![1, 2, 2], vec![5]];
    assert!(compare_2d_unsorted(&result, &expected));
}
