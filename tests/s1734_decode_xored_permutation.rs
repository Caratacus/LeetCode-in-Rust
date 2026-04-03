// Tests for Problem 1734: Decode XOR Permutation
// Java reference: src/test/java/g1701_1800/s1734_decode_xored_permutation/SolutionTest.java

use leetcode_in_rust::s1734::decode_xored_permutation::Solution;

#[test]
fn test_decode() {
    assert_eq!(Solution::decode(vec![3, 1]), vec![1, 2, 3]);
}

#[test]
fn test_decode2() {
    assert_eq!(Solution::decode(vec![6, 5, 4, 6]), vec![2, 4, 1, 5, 3]);
}
