// Tests for Problem 2864: Maximum Odd Binary Number
// Java reference: src/test/java/g2801_2900/s2864_maximum_odd_binary_number/SolutionTest.java

use leetcode_in_rust::s2864::maximum_odd_binary_number::Solution;

#[test]
fn test_maximum_odd_binary_number() {
    assert_eq!(Solution::maximum_odd_binary_number("010".to_string()), "001");
}

#[test]
fn test_maximum_odd_binary_number2() {
    assert_eq!(Solution::maximum_odd_binary_number("0101".to_string()), "1001");
}
