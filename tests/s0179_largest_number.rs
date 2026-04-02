// Tests for Problem 0179: Largest Number
// Java reference: src/test/java/g0121_0200/s0179_largest_number/SolutionTest.java

use leetcode_in_rust::s0179::largest_number::Solution;

#[test]
fn test_largest_number() {
    assert_eq!(Solution::largest_number(vec![10, 2]), String::from("210"));
}

#[test]
fn test_largest_number2() {
    assert_eq!(Solution::largest_number(vec![3, 30, 34, 5, 9]), String::from("9534330"));
}

#[test]
fn test_largest_number3() {
    assert_eq!(Solution::largest_number(vec![0, 0]), String::from("0"));
}
