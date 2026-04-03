// Tests for Problem 1558: Minimum Numbers of Function Calls to Make Target Array
// Java reference: src/test/java/g1501_1600/s1558_minimum_numbers_of_function_calls_to_make_target_array/SolutionTest.java

use leetcode_in_rust::s1558::minimum_numbers_of_function_calls_to_make_target_array::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![1, 5]), 5);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![2, 2]), 3);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![4, 2, 5]), 6);
}
