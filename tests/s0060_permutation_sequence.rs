// Tests for Problem 0060: Permutation Sequence
// Java reference: src/test/java/g0001_0100/s0060_permutation_sequence/SolutionTest.java

use leetcode_in_rust::s0060::permutation_sequence::Solution;

#[test]
fn test_get_permutation() {
    assert_eq!(Solution::get_permutation(3, 3), "213");
}

#[test]
fn test_get_permutation2() {
    assert_eq!(Solution::get_permutation(4, 9), "2314");
}

#[test]
fn test_get_permutation3() {
    assert_eq!(Solution::get_permutation(3, 1), "123");
}
