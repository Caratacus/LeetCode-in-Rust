// Tests for Problem 1668: Maximum Repeating Substring
// Java reference: src/test/java/g1601_1700/s1668_maximum_repeating_substring/SolutionTest.java

use leetcode_in_rust::s1668::maximum_repeating_substring::Solution;

#[test]
fn test_max_repeating() {
    assert_eq!(Solution::max_repeating("ababc".to_string(), "ab".to_string()), 2);
}

#[test]
fn test_max_repeating2() {
    assert_eq!(Solution::max_repeating("ababc".to_string(), "ba".to_string()), 1);
}

#[test]
fn test_max_repeating3() {
    assert_eq!(Solution::max_repeating("ababc".to_string(), "ac".to_string()), 0);
}
