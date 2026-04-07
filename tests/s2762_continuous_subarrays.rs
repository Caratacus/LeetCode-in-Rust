// Tests for Problem 2762: Continuous Subarrays
// Java reference: src/test/java/g2701_2800/s2762_continuous_subarrays/SolutionTest.java

use leetcode_in_rust::s2762::continuous_subarrays::Solution;

#[test]
fn test_continuous_subarrays() {
    assert_eq!(Solution::continuous_subarrays(vec![5, 4, 2, 4]), 8);
}

#[test]
fn test_continuous_subarrays2() {
    assert_eq!(Solution::continuous_subarrays(vec![1, 2, 3]), 6);
}
