// Tests for Problem 3420: Count Non Decreasing Subarrays After K Operations
// Java reference: src/test/java/g3401_3500/s3420_count_non_decreasing_subarrays_after_k_operations/SolutionTest.java

use leetcode_in_rust::s3420::count_non_decreasing_subarrays_after_k_operations::Solution;

#[test]
fn test_count_non_decreasing_subarrays() {
    assert_eq!(Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 2, 4, 4], 7), 17i64);
}

#[test]
fn test_count_non_decreasing_subarrays2() {
    assert_eq!(Solution::count_non_decreasing_subarrays(vec![6, 3, 1, 3, 6], 4), 12i64);
}
