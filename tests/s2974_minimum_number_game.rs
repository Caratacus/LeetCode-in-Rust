// Tests for Problem 2974: Minimum Number Game
// Java reference: src/test/java/g2901_3000/s2974_minimum_number_game/SolutionTest.java

use leetcode_in_rust::s2974::minimum_number_game::Solution;

#[test]
fn test_number_game() {
    assert_eq!(Solution::number_game(vec![5, 4, 2, 3]), vec![3, 2, 5, 4]);
}

#[test]
fn test_number_game2() {
    assert_eq!(Solution::number_game(vec![2, 5]), vec![5, 2]);
}
