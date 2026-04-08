// Tests for Problem 3326: Minimum Division Operations to Make Array Non Decreasing
// Java reference: src/test/java/g3301_3400/s3326_minimum_division_operations_to_make_array_non_decreasing/SolutionTest.java

use leetcode_in_rust::s3326::minimum_division_operations_to_make_array_non_decreasing::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![25, 7]), 1);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![7, 7, 6]), -1);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![1, 1, 1, 1]), 0);
}
