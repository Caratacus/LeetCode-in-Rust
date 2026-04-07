// Tests for Problem 2831: Find the Longest Equal Subarray
// Java reference: src/test/java/g2801_2900/s2831_find_the_longest_equal_subarray/SolutionTest.java

use leetcode_in_rust::s2831::find_the_longest_equal_subarray::Solution;

#[test]
fn test_longest_equal_subarray() {
    assert_eq!(Solution::longest_equal_subarray(vec![1, 3, 2, 3, 1, 3], 3), 3);
}

#[test]
fn test_longest_equal_subarray2() {
    assert_eq!(Solution::longest_equal_subarray(vec![1, 1, 2, 2, 1, 1], 2), 4);
}
