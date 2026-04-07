// Tests for Problem 1823: Find the Winner of the Circular Game
// Java reference: src/test/java/g1801_1900/s1823_find_the_winner_of_the_circular_game/SolutionTest.java

use leetcode_in_rust::s1823::find_the_winner_of_the_circular_game::Solution;

#[test]
fn test_find_the_winner() {
    assert_eq!(Solution::find_the_winner(5, 2), 3);
}

#[test]
fn test_find_the_winner2() {
    assert_eq!(Solution::find_the_winner(6, 5), 1);
}
