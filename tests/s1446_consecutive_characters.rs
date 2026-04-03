// Tests for Problem 1446: Consecutive Characters
// Java reference: src/test/java/g1401_1500/s1446_consecutive_characters/SolutionTest.java

use leetcode_in_rust::s1446::consecutive_characters::Solution;

#[test]
fn test_max_power() {
    assert_eq!(Solution::max_power("leetcode".to_string()), 2);
}

#[test]
fn test_max_power2() {
    assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
}
