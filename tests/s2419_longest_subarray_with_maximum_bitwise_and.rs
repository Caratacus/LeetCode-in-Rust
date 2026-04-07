// Tests for Problem 2419: Longest Subarray With Maximum Bitwise AND
// Java reference: src/test/java/g2401_2500/s2419_longest_subarray_with_maximum_bitwise_and/SolutionTest.java

use leetcode_in_rust::s2419::longest_subarray_with_maximum_bitwise_and::Solution;

#[test]
fn test_longest_subarray() {
    assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 3, 2, 2]), 1);
}

#[test]
fn test_longest_subarray2() {
    assert_eq!(Solution::longest_subarray(vec![1, 2, 3, 4]), 1);
}
