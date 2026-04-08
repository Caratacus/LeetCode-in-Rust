// Tests for Problem 3101: Count Alternating Subarrays
// Java reference: src/test/java/g3101_3200/s3101_count_alternating_subarrays/SolutionTest.java

use leetcode_in_rust::s3101::count_alternating_subarrays::Solution;

#[test]
fn test_count_alternating_subarrays() {
    assert_eq!(Solution::count_alternating_subarrays(vec![0, 1, 1, 1]), 5);
}

#[test]
fn test_count_alternating_subarrays2() {
    assert_eq!(Solution::count_alternating_subarrays(vec![1, 0, 1, 0]), 10);
}
