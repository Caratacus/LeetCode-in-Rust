// Tests for Problem 2488: Count Subarrays With Median K
// Java reference: src/test/java/g2401_2500/s2488_count_subarrays_with_median_k/SolutionTest.java

use leetcode_in_rust::s2488::count_subarrays_with_median_k::Solution;

#[test]
fn test_count_subarrays() {
    assert_eq!(Solution::count_subarrays(vec![3, 2, 1, 4, 5], 4), 3);
}

#[test]
fn test_count_subarrays2() {
    assert_eq!(Solution::count_subarrays(vec![2, 3, 1], 3), 1);
}
