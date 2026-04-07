// Tests for Problem 2935: Maximum Strong Pair XOR II
// Java reference: src/test/java/g2901_3000/s2935_maximum_strong_pair_xor_ii/SolutionTest.java

use leetcode_in_rust::s2935::maximum_strong_pair_xor_ii::Solution;

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
    assert_eq!(Solution::maximum_strong_pair_xor(vec![500, 520, 2500, 3000]), 1020);
}
