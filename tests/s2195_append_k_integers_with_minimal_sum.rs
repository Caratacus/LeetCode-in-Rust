// Tests for Problem 2195: Append K Integers With Minimal Sum
// Java reference: src/test/java/g2101_2200/s2195_append_k_integers_with_minimal_sum/SolutionTest.java

use leetcode_in_rust::s2195::append_k_integers_with_minimal_sum::Solution;

#[test]
fn test_minimal_k_sum() {
    assert_eq!(Solution::minimal_k_sum(vec![1, 4, 25, 10, 25], 2), 5);
}

#[test]
fn test_minimal_k_sum2() {
    assert_eq!(Solution::minimal_k_sum(vec![5, 6], 6), 25);
}
