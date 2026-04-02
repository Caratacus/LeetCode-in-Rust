// Tests for Problem 0039: Combination Sum
// Java reference: src/test/java/g0001_0100/s0039_combination_sum/SolutionTest.java

use leetcode_in_rust::s0039::combination_sum::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_combination_sum() {
    let result = Solution::combination_sum(vec![2, 3, 6, 7], 7);
    let expected = vec![vec![7], vec![2, 2, 3]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_combination_sum2() {
    let result = Solution::combination_sum(vec![2, 3, 5], 8);
    let expected = vec![vec![3, 5], vec![2, 3, 3], vec![2, 2, 2, 2]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_combination_sum3() {
    let result = Solution::combination_sum(vec![2], 1);
    let expected: Vec<Vec<i32>> = vec![];
    assert!(compare_2d_unsorted(&result, &expected));
}
