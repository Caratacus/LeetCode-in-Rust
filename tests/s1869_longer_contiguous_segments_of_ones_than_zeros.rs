// Tests for Problem 1869: Longer Contiguous Segments of Ones than Zeros
// Java reference: src/test/java/g1801_1900/s1869_longer_contiguous_segments_of_ones_than_zeros/SolutionTest.java

use leetcode_in_rust::s1869::longer_contiguous_segments_of_ones_than_zeros::Solution;

#[test]
fn test_check_zero_ones() {
    assert_eq!(Solution::check_zero_ones("1101".to_string()), true);
}

#[test]
fn test_check_zero_ones2() {
    assert_eq!(Solution::check_zero_ones("111000".to_string()), false);
}

#[test]
fn test_check_zero_ones3() {
    assert_eq!(Solution::check_zero_ones("110100010".to_string()), false);
}
