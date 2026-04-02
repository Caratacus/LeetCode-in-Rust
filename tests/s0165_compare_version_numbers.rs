// Tests for Problem 0165: Compare Version Numbers
// Java reference: src/test/java/g0121_0200/s0165_compare_version_numbers/SolutionTest.java

use leetcode_in_rust::s0165::compare_version_numbers::Solution;

#[test]
fn test_compare_version() {
    assert_eq!(Solution::compare_version(String::from("1.01"), String::from("1.001")), 0);
}

#[test]
fn test_compare_version2() {
    assert_eq!(Solution::compare_version(String::from("1.0"), String::from("1.0.0")), 0);
}

#[test]
fn test_compare_version3() {
    assert_eq!(Solution::compare_version(String::from("0.1"), String::from("1.1")), -1);
}
