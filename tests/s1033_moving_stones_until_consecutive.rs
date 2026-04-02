// Tests for Problem 1033: Moving Stones Until Consecutive
// Java reference: src/test/java/g1001_1100/s1033_moving_stones_until_consecutive/SolutionTest.java

use leetcode_in_rust::s1033::moving_stones_until_consecutive::Solution;

#[test]
fn test_num_moves_stones() {
    assert_eq!(Solution::num_moves_stones(1, 2, 5), vec![1, 2]);
}

#[test]
fn test_num_moves_stones2() {
    assert_eq!(Solution::num_moves_stones(4, 3, 2), vec![0, 0]);
}

#[test]
fn test_num_moves_stones3() {
    assert_eq!(Solution::num_moves_stones(3, 5, 1), vec![1, 2]);
}
