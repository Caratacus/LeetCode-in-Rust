// Tests for Problem 2401: Longest Nice Subarray
// Java reference: src/test/java/g2401_2500/s2401_longest_nice_subarray/SolutionTest.java

use leetcode_in_rust::s2401::longest_nice_subarray::Solution;

#[test]
fn test_longest_nice_subarray() {
    assert_eq!(Solution::longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
}

#[test]
fn test_longest_nice_subarray2() {
    assert_eq!(Solution::longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
}
