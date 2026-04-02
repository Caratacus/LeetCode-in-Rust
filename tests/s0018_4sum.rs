// Tests for Problem 0018: 4Sum
// Java reference: src/test/java/g0001_0100/s0018_4sum/SolutionTest.java

use leetcode_in_rust::s0018::p4sum::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_four_sum() {
    let result = Solution::four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    let expected = vec![vec![-2, -1, 1, 2], vec![-2, 0, 0, 2], vec![-1, 0, 0, 1]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_four_sum2() {
    let result = Solution::four_sum(vec![2, 2, 2, 2, 2], 8);
    let expected = vec![vec![2, 2, 2, 2]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_four_sum3() {
    let result = Solution::four_sum(vec![2, 2, 2], 8);
    let expected: Vec<Vec<i32>> = vec![];
    assert!(compare_2d_unsorted(&result, &expected));
}
