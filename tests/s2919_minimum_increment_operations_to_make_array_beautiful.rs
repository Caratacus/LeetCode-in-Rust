// Tests for Problem 2919: Minimum Increment Operations to Make Array Beautiful
// Java reference: src/test/java/g2901_3000/s2919_minimum_increment_operations_to_make_array_beautiful/SolutionTest.java

use leetcode_in_rust::s2919::minimum_increment_operations_to_make_array_beautiful::Solution;

#[test]
fn test_min_increment_operations() {
    assert_eq!(Solution::min_increment_operations(vec![2, 3, 0, 0, 2], 4), 3);
}

#[test]
fn test_min_increment_operations2() {
    assert_eq!(Solution::min_increment_operations(vec![0, 1, 3, 3], 5), 2);
}

#[test]
fn test_min_increment_operations3() {
    assert_eq!(Solution::min_increment_operations(vec![1, 1, 2], 1), 0);
}
