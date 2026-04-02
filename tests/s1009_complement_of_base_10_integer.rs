// Tests for Problem 1009: Complement of Base 10 Integer
// Java reference: src/test/java/g1001_1100/s1009_complement_of_base_10_integer/SolutionTest.java

use leetcode_in_rust::s1009::complement_of_base_10_integer::Solution;

#[test]
fn test_bitwise_complement() {
    assert_eq!(Solution::bitwise_complement(5), 2);
}

#[test]
fn test_bitwise_complement2() {
    assert_eq!(Solution::bitwise_complement(7), 0);
}

#[test]
fn test_bitwise_complement3() {
    assert_eq!(Solution::bitwise_complement(10), 5);
}
