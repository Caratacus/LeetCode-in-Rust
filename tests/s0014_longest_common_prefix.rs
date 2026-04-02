// Tests for Problem 0014: Longest Common Prefix
// Java reference: src/test/java/g0001_0100/s0014_longest_common_prefix/SolutionTest.java

use leetcode_in_rust::s0014::longest_common_prefix::Solution;

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]),
        "fl"
    );
}

#[test]
fn test_longest_common_prefix2() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]),
        ""
    );
}
