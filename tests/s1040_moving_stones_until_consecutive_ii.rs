// Tests for Problem 1040: Moving Stones Until Consecutive II
// Java reference: src/test/java/g1001_1100/s1040_moving_stones_until_consecutive_ii/SolutionTest.java

use leetcode_in_rust::s1040::moving_stones_until_consecutive_ii::Solution;

#[test]
fn test_num_moves_stones_ii() {
    assert_eq!(Solution::num_moves_stones_ii(vec![7, 4, 9]), vec![1, 2]);
}

#[test]
fn test_num_moves_stones_ii2() {
    assert_eq!(Solution::num_moves_stones_ii(vec![6, 5, 4, 3, 10]), vec![2, 3]);
}
