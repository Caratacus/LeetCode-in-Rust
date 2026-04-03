// Tests for Problem 1566: Detect Pattern of Length M Repeated K or More Times
// Java reference: src/test/java/g1501_1600/s1566_detect_pattern_of_length_m_repeated_k_or_more_times/SolutionTest.java

use leetcode_in_rust::s1566::detect_pattern_of_length_m_repeated_k_or_more_times::Solution;

#[test]
fn test_contains_pattern() {
    assert!(Solution::contains_pattern(vec![1, 2, 4, 4, 4, 4], 1, 3));
}

#[test]
fn test_contains_pattern2() {
    assert!(Solution::contains_pattern(vec![1, 2, 1, 2, 1, 1, 1, 3], 2, 2));
}

#[test]
fn test_contains_pattern3() {
    assert!(!Solution::contains_pattern(vec![1, 2, 1, 2, 1, 3], 2, 3));
}
