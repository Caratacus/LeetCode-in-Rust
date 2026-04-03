// Tests for Problem 1680: Concatenation of Consecutive Binary Numbers
// Java reference: src/test/java/g1601_1700/s1680_concatenation_of_consecutive_binary_numbers/SolutionTest.java

use leetcode_in_rust::s1680::concatenation_of_consecutive_binary_numbers::Solution;

#[test]
fn test_concatenated_binary() {
    assert_eq!(Solution::concatenated_binary(1), 1);
}

#[test]
fn test_concatenated_binary2() {
    assert_eq!(Solution::concatenated_binary(3), 27);
}

#[test]
fn test_concatenated_binary3() {
    assert_eq!(Solution::concatenated_binary(12), 505379714);
}
