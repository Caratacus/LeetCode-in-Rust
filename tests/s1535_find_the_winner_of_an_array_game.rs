// Tests for Problem 1535: Find the Winner of an Array Game
// Java reference: src/test/java/g1501_1600/s1535_find_the_winner_of_an_array_game/SolutionTest.java

use leetcode_in_rust::s1535::find_the_winner_of_an_array_game::Solution;

#[test]
fn test_get_winner() {
    assert_eq!(Solution::get_winner(vec![2, 1, 3, 5, 4, 6, 7], 2), 5);
}

#[test]
fn test_get_winner2() {
    assert_eq!(Solution::get_winner(vec![3, 2, 1], 10), 3);
}
