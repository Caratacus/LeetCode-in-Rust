// Tests for Problem 0629: K Inverse Pairs Array
// Java reference: src/test/java/g0601_0700/s0629_k_inverse_pairs_array/SolutionTest.java

use leetcode_in_rust::s0629::k_inverse_pairs_array::Solution;

#[test]
fn test_k_inverse_pairs() {
    assert_eq!(Solution::k_inverse_pairs(3, 0), 1);
}

#[test]
fn test_k_inverse_pairs2() {
    assert_eq!(Solution::k_inverse_pairs(3, 1), 2);
}
