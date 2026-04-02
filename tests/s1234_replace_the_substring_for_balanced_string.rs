// Tests for Problem 1234: Replace the Substring for Balanced String
// Java reference: src/test/java/g1201_1300/s1234_replace_the_substring_for_balanced_string/SolutionTest.java

use leetcode_in_rust::s1234::replace_the_substring_for_balanced_string::Solution;

#[test]
fn test_balanced_string() {
    assert_eq!(Solution::balanced_string("QWER".to_string()), 0);
}

#[test]
fn test_balanced_string2() {
    assert_eq!(Solution::balanced_string("QQWE".to_string()), 1);
}

#[test]
fn test_balanced_string3() {
    assert_eq!(Solution::balanced_string("QQQW".to_string()), 2);
}
