// Tests for Problem 1509: Minimum Difference Between Largest and Smallest Value in Three Moves
// Java reference: src/test/java/g1501_1600/s1509_minimum_difference_between_largest_and_smallest_value_in_three_moves/SolutionTest.java

use leetcode_in_rust::s1509::minimum_difference_between_largest_and_smallest_value_in_three_moves::Solution;

#[test]
fn test_min_difference() {
    assert_eq!(Solution::min_difference(vec![5, 3, 2, 4]), 0);
}

#[test]
fn test_min_difference2() {
    assert_eq!(Solution::min_difference(vec![1, 5, 0, 10, 14]), 1);
}
