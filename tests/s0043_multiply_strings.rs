// Tests for Problem 0043: Multiply Strings
// Java reference: src/test/java/g0001_0100/s0043_multiply_strings/SolutionTest.java

use leetcode_in_rust::s0043::multiply_strings::Solution;

#[test]
fn test_multiply() {
    assert_eq!(Solution::multiply("2".to_string(), "3".to_string()), "6");
}

#[test]
fn test_multiply2() {
    assert_eq!(Solution::multiply("123".to_string(), "456".to_string()), "56088");
}
