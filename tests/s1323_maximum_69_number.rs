// Tests for Problem 1323: Maximum 69 Number
// Java reference: src/test/java/g1301_1400/s1323_maximum_69_number/SolutionTest.java

use leetcode_in_rust::s1323::maximum_69_number::Solution;

#[test]
fn test_maximum69_number() {
    assert_eq!(Solution::maximum69_number(9996), 9999);
}

#[test]
fn test_maximum69_number2() {
    assert_eq!(Solution::maximum69_number(9999), 9999);
}
