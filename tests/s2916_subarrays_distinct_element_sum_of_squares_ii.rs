// Tests for Problem 2916: Subarrays Distinct Element Sum of Squares II
// Java reference: src/test/java/g2901_3000/s2916_subarrays_distinct_element_sum_of_squares_ii/SolutionTest.java

use leetcode_in_rust::s2916::subarrays_distinct_element_sum_of_squares_ii::Solution;

#[test]
fn test_sum_counts() {
    assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
}

#[test]
fn test_sum_counts2() {
    assert_eq!(Solution::sum_counts(vec![2, 2]), 3);
}
