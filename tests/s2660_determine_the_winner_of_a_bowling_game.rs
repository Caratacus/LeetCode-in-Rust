// Tests for Problem 2660: Determine the Winner of a Bowling Game
// Java reference: src/test/java/g2601_2700/s2660_determine_the_winner_of_a_bowling_game/SolutionTest.java

use leetcode_in_rust::s2660::determine_the_winner_of_a_bowling_game::Solution;

#[test]
fn test_is_winner() {
    assert_eq!(
        Solution::is_winner(vec![4, 10, 7, 9], vec![6, 5, 2, 3]),
        1
    );
}

#[test]
fn test_is_winner2() {
    assert_eq!(
        Solution::is_winner(vec![3, 5, 7, 6], vec![8, 10, 10, 2]),
        2
    );
}

#[test]
fn test_is_winner3() {
    assert_eq!(Solution::is_winner(vec![2, 3], vec![4, 1]), 0);
}
