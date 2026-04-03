// Tests for Problem 1674: Minimum Moves to Make Array Complementary
// Java reference: src/test/java/g1601_1700/s1674_minimum_moves_to_make_array_complementary/SolutionTest.java

use leetcode_in_rust::s1674::minimum_moves_to_make_array_complementary::Solution;

#[test]
fn test_min_moves() {
    assert_eq!(Solution::min_moves(vec![1, 2, 4, 3], 4), 1);
}

#[test]
fn test_min_moves2() {
    assert_eq!(Solution::min_moves(vec![1, 2, 2, 1], 2), 2);
}

#[test]
fn test_min_moves3() {
    assert_eq!(Solution::min_moves(vec![1, 2, 1, 2], 2), 0);
}
