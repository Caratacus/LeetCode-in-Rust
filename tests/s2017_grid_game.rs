// Tests for Problem 2017: Grid Game
// Java reference: src/test/java/g2001_2100/s2017_grid_game/SolutionTest.java

use leetcode_in_rust::s2017::grid_game::Solution;

#[test]
fn test_grid_game() {
    assert_eq!(Solution::grid_game(vec![vec![2, 5, 4], vec![1, 5, 1]]), 4);
}

#[test]
fn test_grid_game2() {
    assert_eq!(Solution::grid_game(vec![vec![3, 3, 1], vec![8, 5, 2]]), 4);
}

#[test]
fn test_grid_game3() {
    assert_eq!(
        Solution::grid_game(vec![vec![1, 3, 1, 15], vec![1, 3, 3, 1]]),
        7
    );
}
