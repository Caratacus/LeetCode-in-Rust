// Tests for Problem 2980: Check if Bitwise OR Has Trailing Zeros
// Java reference: src/test/java/g2901_3000/s2980_check_if_bitwise_or_has_trailing_zeros/SolutionTest.java

use leetcode_in_rust::s2980::check_if_bitwise_or_has_trailing_zeros::Solution;

#[test]
fn test_has_trailing_zeros() {
    assert_eq!(Solution::has_trailing_zeros(vec![1, 2, 3, 4, 5]), true);
}

#[test]
fn test_has_trailing_zeros2() {
    assert_eq!(Solution::has_trailing_zeros(vec![2, 4, 8, 16]), true);
}

#[test]
fn test_has_trailing_zeros3() {
    assert_eq!(Solution::has_trailing_zeros(vec![1, 3, 5, 7, 9]), false);
}
