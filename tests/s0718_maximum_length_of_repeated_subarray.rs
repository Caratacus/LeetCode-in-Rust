// Tests for Problem 0718: Maximum Length of Repeated Subarray
// Java reference: src/test/java/g0701_0800/s0718_maximum_length_of_repeated_subarray/SolutionTest.java

use leetcode_in_rust::s0718::maximum_length_of_repeated_subarray::Solution;

#[test]
fn test_find_length() {
    assert_eq!(
        Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7]),
        3
    );
}

#[test]
fn test_find_length2() {
    assert_eq!(
        Solution::find_length(vec![0, 0, 0, 0, 0], vec![0, 0, 0, 0, 0]),
        5
    );
}
