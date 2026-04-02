// Tests for Problem 0581: Shortest Unsorted Continuous Subarray
// Java reference: src/test/java/g0501_0600/s0581_shortest_unsorted_continuous_subarray/SolutionTest.java

use leetcode_in_rust::s0581::shortest_unsorted_continuous_subarray::Solution;

#[test]
fn test_find_unsorted_subarray() {
    assert_eq!(Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15]), 5);
}

#[test]
fn test_find_unsorted_subarray2() {
    assert_eq!(Solution::find_unsorted_subarray(vec![1, 2, 3, 4]), 0);
}

#[test]
fn test_find_unsorted_subarray3() {
    assert_eq!(Solution::find_unsorted_subarray(vec![1]), 0);
}
