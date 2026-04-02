// Tests for Problem 0015: 3Sum
// Java reference: src/test/java/g0001_0100/s0015_3sum/SolutionTest.java

use leetcode_in_rust::s0015::p3sum::Solution;
use leetcode_in_rust::utils::array_utils::compare_2d_unsorted;

#[test]
fn test_three_sum() {
    let result = Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]);
    let expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_three_sum2() {
    let result = Solution::three_sum(vec![]);
    let expected: Vec<Vec<i32>> = vec![];
    assert!(compare_2d_unsorted(&result, &expected));
}

#[test]
fn test_three_sum3() {
    let result = Solution::three_sum(vec![0]);
    let expected: Vec<Vec<i32>> = vec![];
    assert!(compare_2d_unsorted(&result, &expected));
}
