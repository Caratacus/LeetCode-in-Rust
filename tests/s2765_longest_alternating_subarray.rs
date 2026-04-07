// Tests for Problem 2765: Longest Alternating Subarray
// Java reference: src/test/java/g2701_2800/s2765_longest_alternating_subarray/SolutionTest.java

use leetcode_in_rust::s2765::longest_alternating_subarray::Solution;

#[test]
fn test_alternating_subarray() {
    assert_eq!(Solution::alternating_subarray(vec![2, 3, 4, 3, 4]), 4);
}

#[test]
fn test_alternating_subarray2() {
    assert_eq!(Solution::alternating_subarray(vec![4, 5, 6]), 2);
}
