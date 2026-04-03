// Tests for Problem 0822: Card Flipping Game
// Java reference: src/test/java/g0801_0900/s0822_card_flipping_game/SolutionTest.java

use leetcode_in_rust::s0822::card_flipping_game::Solution;

#[test]
fn test_flipgame() {
    assert_eq!(Solution::flipgame(vec![1, 2, 4, 4, 7], vec![1, 3, 4, 1, 3]), 2);
}

#[test]
fn test_flipgame2() {
    assert_eq!(Solution::flipgame(vec![1], vec![1]), 0);
}
