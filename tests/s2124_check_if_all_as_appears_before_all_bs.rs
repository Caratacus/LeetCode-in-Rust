// Tests for Problem 2124: Check if All A's Appears Before All B's
// Java reference: src/test/java/g2101_2200/s2124_check_if_all_as_appears_before_all_bs/SolutionTest.java

use leetcode_in_rust::s2124::check_if_all_as_appears_before_all_bs::Solution;

#[test]
fn test_check_string() {
    assert_eq!(Solution::check_string("aaabbb".to_string()), true);
}

#[test]
fn test_check_string2() {
    assert_eq!(Solution::check_string("abab".to_string()), false);
}

#[test]
fn test_check_string3() {
    assert_eq!(Solution::check_string("bbb".to_string()), true);
}
