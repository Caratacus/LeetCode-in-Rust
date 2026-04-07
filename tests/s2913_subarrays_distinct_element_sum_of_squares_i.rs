// Tests for Problem 2913: Subarrays Distinct Element Sum of Squares I
// Java reference: src/test/java/g2901_3000/s2913_subarrays_distinct_element_sum_of_squares_i/SolutionTest.java

use leetcode_in_rust::s2913::subarrays_distinct_element_sum_of_squares_i::Solution;

#[test]
fn test_sum_counts() {
    assert_eq!(Solution::sum_counts(vec![1, 2, 1]), 15);
}

#[test]
fn test_sum_counts2() {
    assert_eq!(Solution::sum_counts(vec![1, 1]), 3);
}
