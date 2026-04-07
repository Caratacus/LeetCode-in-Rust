// Tests for Problem 2317: Maximum XOR After Operations
// Java reference: src/test/java/g2301_2400/s2317_maximum_xor_after_operations/SolutionTest.java

use leetcode_in_rust::s2317::maximum_xor_after_operations::Solution;

#[test]
fn test_maximum_xor() {
    assert_eq!(Solution::maximum_xor(vec![3, 2, 4, 6]), 7);
}

#[test]
fn test_maximum_xor2() {
    assert_eq!(Solution::maximum_xor(vec![1, 2, 3, 9, 2]), 11);
}
