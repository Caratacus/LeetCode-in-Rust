// Tests for Problem 2656: Maximum Sum With Exactly K Elements
// Java reference: src/test/java/g2601_2700/s2656_maximum_sum_with_exactly_k_elements/SolutionTest.java

use leetcode_in_rust::s2656::maximum_sum_with_exactly_k_elements::Solution;

#[test]
fn test_maximize_sum() {
    assert_eq!(Solution::maximize_sum(vec![1, 2, 3, 4, 5], 3), 18);
}

#[test]
fn test_maximize_sum2() {
    assert_eq!(Solution::maximize_sum(vec![5, 5, 5], 2), 11);
}
