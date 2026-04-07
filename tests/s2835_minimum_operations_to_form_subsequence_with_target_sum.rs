// Tests for Problem 2835: Minimum Operations to Form Subsequence With Target Sum
// Java reference: src/test/java/g2801_2900/s2835_minimum_operations_to_form_subsequence_with_target_sum/SolutionTest.java

use leetcode_in_rust::s2835::minimum_operations_to_form_subsequence_with_target_sum::Solution;

#[test]
fn test_min_operations() {
    assert_eq!(Solution::min_operations(vec![1, 2, 8], 7), 1);
}

#[test]
fn test_min_operations2() {
    assert_eq!(Solution::min_operations(vec![1, 32, 1, 2], 12), 2);
}

#[test]
fn test_min_operations3() {
    assert_eq!(Solution::min_operations(vec![1, 32, 1, 2], 12), 2);
}
