// Tests for Problem 0004: Median of Two Sorted Arrays
// Java reference: src/test/java/g0001_0100/s0004_median_of_two_sorted_arrays/SolutionTest.java

use leetcode_in_rust::s0004::median_of_two_sorted_arrays::Solution;

#[test]
fn test_find_median_sorted_arrays() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
}

#[test]
fn test_find_median_sorted_arrays2() {
    assert_eq!(Solution::find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
}
