// Tests for Problem 1653: Minimum Deletions to Make String Balanced
// Java reference: src/test/java/g1601_1700/s1653_minimum_deletions_to_make_string_balanced/SolutionTest.java

use leetcode_in_rust::s1653::minimum_deletions_to_make_string_balanced::Solution;

#[test]
fn test_minimum_deletions() {
    assert_eq!(Solution::minimum_deletions("aababbab".to_string()), 2);
}

#[test]
fn test_minimum_deletions2() {
    assert_eq!(Solution::minimum_deletions("bbaaaaabb".to_string()), 2);
}
