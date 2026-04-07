// Tests for Problem 2771: Longest Non Decreasing Subarray From Two Arrays
// Java reference: src/test/java/g2701_2800/s2771_longest_non_decreasing_subarray_from_two_arrays/SolutionTest.java

use leetcode_in_rust::s2771::longest_non_decreasing_subarray_from_two_arrays::Solution;

#[test]
fn test_max_non_decreasing_length() {
    assert_eq!(
        Solution::max_non_decreasing_length(vec![2, 3, 1], vec![1, 2, 1]),
        2
    );
}

#[test]
fn test_max_non_decreasing_length2() {
    assert_eq!(
        Solution::max_non_decreasing_length(vec![1, 3, 2, 1], vec![2, 2, 3, 4]),
        4
    );
}
