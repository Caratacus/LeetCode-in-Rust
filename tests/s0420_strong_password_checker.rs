// Tests for Problem 0420: Strong Password Checker
// Java reference: src/test/java/g0401_0500/s0420_strong_password_checker/SolutionTest.java

use leetcode_in_rust::s0420::strong_password_checker::Solution;

#[test]
fn test_strong_password_checker() {
    assert_eq!(Solution::strong_password_checker("a".to_string()), 5);
}

#[test]
fn test_strong_password_checker2() {
    assert_eq!(Solution::strong_password_checker("aA1".to_string()), 3);
}

#[test]
fn test_strong_password_checker3() {
    assert_eq!(Solution::strong_password_checker("1337C0d3".to_string()), 0);
}
