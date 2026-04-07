// Tests for Problem 2272: Substring With Largest Variance
// Java reference: src/test/java/g2201_2300/s2272_substring_with_largest_variance/SolutionTest.java

use leetcode_in_rust::s2272::substring_with_largest_variance::Solution;

#[test]
fn test_largest_variance() {
    assert_eq!(Solution::largest_variance("aababbb".to_string()), 3);
}

#[test]
fn test_largest_variance2() {
    assert_eq!(Solution::largest_variance("abcde".to_string()), 0);
}
