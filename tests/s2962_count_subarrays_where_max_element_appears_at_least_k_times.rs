// Tests for Problem 2962: Count Subarrays Where Max Element Appears at Least K Times
// Java reference: src/test/java/g2901_3000/s2962_count_subarrays_where_max_element_appears_at_least_k_times/SolutionTest.java

use leetcode_in_rust::s2962::count_subarrays_where_max_element_appears_at_least_k_times::Solution;

#[test]
fn test_count_subarrays() {
    assert_eq!(Solution::count_subarrays(vec![1, 3, 2, 3, 3], 2), 6);
}

#[test]
fn test_count_subarrays2() {
    assert_eq!(Solution::count_subarrays(vec![1, 4, 2, 1], 3), 0);
}
