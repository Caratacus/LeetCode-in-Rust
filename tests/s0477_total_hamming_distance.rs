// Tests for Problem 0477: Total Hamming Distance
// Java reference: src/test/java/g0401_0500/s0477_total_hamming_distance/SolutionTest.java

use leetcode_in_rust::s0477::total_hamming_distance::Solution;

#[test]
fn test_total_hamming_distance() {
    assert_eq!(Solution::total_hamming_distance(vec![4, 14, 2]), 6);
}

#[test]
fn test_total_hamming_distance2() {
    assert_eq!(Solution::total_hamming_distance(vec![4, 14, 4]), 4);
}
