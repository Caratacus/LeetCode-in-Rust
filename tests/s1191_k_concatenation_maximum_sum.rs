// Tests for Problem 1191: K-Concatenation Maximum Sum
// Java reference: src/test/java/g1101_1200/s1191_k_concatenation_maximum_sum/SolutionTest.java

use leetcode_in_rust::s1191::k_concatenation_maximum_sum::Solution;

#[test]
fn test_k_concatenation_max_sum() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, 2], 3), 9);
}

#[test]
fn test_k_concatenation_max_sum2() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![1, -2, 1], 5), 2);
}

#[test]
fn test_k_concatenation_max_sum3() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 7), 0);
}

#[test]
fn test_k_concatenation_max_sum4() {
    assert_eq!(Solution::k_concatenation_max_sum(vec![-1, -2], 1), -1);
}
