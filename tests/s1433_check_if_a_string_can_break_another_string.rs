// Tests for Problem 1433: Check If a String Can Break Another String
// Java reference: src/test/java/g1401_1500/s1433_check_if_a_string_can_break_another_string/SolutionTest.java

use leetcode_in_rust::s1433::check_if_a_string_can_break_another_string::Solution;

#[test]
fn test_check_if_can_break() {
    assert_eq!(Solution::check_if_can_break("abc".to_string(), "xya".to_string()), true);
}

#[test]
fn test_check_if_can_break2() {
    assert_eq!(Solution::check_if_can_break("abe".to_string(), "acd".to_string()), false);
}

#[test]
fn test_check_if_can_break3() {
    assert_eq!(Solution::check_if_can_break("leetcodee".to_string(), "interview".to_string()), true);
}
