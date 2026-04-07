// Tests for Problem 2682: Find the Losers of the Circular Game
// Java reference: src/test/java/g2601_2700/s2682_find_the_losers_of_the_circular_game/SolutionTest.java

use leetcode_in_rust::s2682::find_the_losers_of_the_circular_game::Solution;

#[test]
fn test_circular_game_losers() {
    assert_eq!(Solution::circular_game_losers(5, 2), vec![4, 5]);
}

#[test]
fn test_circular_game_losers2() {
    assert_eq!(Solution::circular_game_losers(4, 4), vec![2, 3, 4]);
}
