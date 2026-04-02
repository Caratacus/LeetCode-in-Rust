// Tests for Problem 0008: String to Integer (atoi)
// Java reference: src/test/java/g0001_0100/s0008_string_to_integer_atoi/SolutionTest.java

use leetcode_in_rust::s0008::string_to_integer_atoi::Solution;

#[test]
fn test_my_atoi() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
}

#[test]
fn test_my_atoi2() {
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
}

#[test]
fn test_my_atoi3() {
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
}

#[test]
fn test_my_atoi4() {
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
}

#[test]
fn test_my_atoi5() {
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
}
