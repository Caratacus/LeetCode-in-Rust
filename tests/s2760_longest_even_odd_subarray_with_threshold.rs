// Tests for Problem 2760: Longest Even Odd Subarray With Threshold
// Java reference: src/test/java/g2701_2800/s2760_longest_even_odd_subarray_with_threshold/SolutionTest.java

use leetcode_in_rust::s2760::longest_even_odd_subarray_with_threshold::Solution;

#[test]
fn test_longest_alternating_subarray() {
    assert_eq!(Solution::longest_alternating_subarray(vec![3, 2, 5, 4], 5), 3);
}

#[test]
fn test_longest_alternating_subarray2() {
    assert_eq!(Solution::longest_alternating_subarray(vec![1, 2], 2), 1);
}

#[test]
fn test_longest_alternating_subarray3() {
    assert_eq!(Solution::longest_alternating_subarray(vec![2, 3, 4, 5], 4), 3);
}
