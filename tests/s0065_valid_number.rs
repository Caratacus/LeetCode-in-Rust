// Tests for Problem 0065: Valid Number
// Java reference: src/test/java/g0001_0100/s0065_valid_number/SolutionTest.java

use leetcode_in_rust::s0065::valid_number::Solution;

#[test]
fn test_is_number() {
    assert_eq!(Solution::is_number("0".to_string()), true);
}

#[test]
fn test_is_number2() {
    assert_eq!(Solution::is_number("e".to_string()), false);
}

#[test]
fn test_is_number3() {
    assert_eq!(Solution::is_number(".".to_string()), false);
}

#[test]
fn test_is_number4() {
    assert_eq!(Solution::is_number(".1".to_string()), true);
}
