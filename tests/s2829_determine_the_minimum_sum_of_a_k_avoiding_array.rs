// Tests for Problem 2829: Determine the Minimum Sum of a k-Avoiding Array
// Java reference: src/test/java/g2801_2900/s2829_determine_the_minimum_sum_of_a_k_avoiding_array/SolutionTest.java

use leetcode_in_rust::s2829::determine_the_minimum_sum_of_a_k_avoiding_array::Solution;

#[test]
fn test_minimum_sum() {
    assert_eq!(Solution::minimum_sum(5, 4), 18);
}

#[test]
fn test_minimum_sum2() {
    assert_eq!(Solution::minimum_sum(2, 6), 3);
}
