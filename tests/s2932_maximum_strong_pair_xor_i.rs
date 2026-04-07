// Tests for Problem 2932: Maximum Strong Pair XOR I
// Java reference: src/test/java/g2901_3000/s2932_maximum_strong_pair_xor_i/SolutionTest.java

use leetcode_in_rust::s2932::maximum_strong_pair_xor_i::Solution;

#[test]
fn test_maximum_strong_pair_xor() {
    assert_eq!(Solution::maximum_strong_pair_xor(vec![1, 2, 3, 4, 5]), 7);
}

#[test]
fn test_maximum_strong_pair_xor2() {
    assert_eq!(Solution::maximum_strong_pair_xor(vec![10, 100]), 0);
}

#[test]
fn test_maximum_strong_pair_xor3() {
    assert_eq!(Solution::maximum_strong_pair_xor(vec![5, 6, 25, 30]), 7);
}
