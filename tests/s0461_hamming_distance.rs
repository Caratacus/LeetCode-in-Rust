// Tests for Problem 0461: Hamming Distance
// Java reference: src/test/java/g0401_0500/s0461_hamming_distance/SolutionTest.java

use leetcode_in_rust::s0461::hamming_distance::Solution;

#[test]
fn test_hamming_distance() {
    assert_eq!(Solution::hamming_distance(1, 4), 2);
}

#[test]
fn test_hamming_distance2() {
    assert_eq!(Solution::hamming_distance(3, 1), 1);
}
