// Tests for Problem 2293: Min Max Game
// Java reference: src/test/java/g2201_2300/s2293_min_max_game/SolutionTest.java

use leetcode_in_rust::s2293::min_max_game::Solution;

#[test]
fn test_min_max_game() {
    assert_eq!(Solution::min_max_game(vec![1, 3, 5, 2, 4, 8, 2, 2]), 1);
}

#[test]
fn test_min_max_game2() {
    assert_eq!(Solution::min_max_game(vec![3]), 3);
}
