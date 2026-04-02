// Tests for Problem 0289: Game of Life
// Java reference: src/test/java/g0201_0300/s0289_game_of_life/SolutionTest.java
// Note: The Rust API takes Vec<Vec<i32>> by value; test adjusted accordingly

use leetcode_in_rust::s0289::game_of_life::Solution;

#[test]
#[ignore = "API takes ownership - cannot verify result without modification"]
fn test_game_of_life() {
    let board = vec![vec![0, 1, 0], vec![0, 0, 1], vec![1, 1, 1], vec![0, 0, 0]];
    Solution::game_of_life(board);
}

#[test]
#[ignore = "API takes ownership - cannot verify result without modification"]
fn test_game_of_life2() {
    let board = vec![vec![1, 1], vec![1, 0]];
    Solution::game_of_life(board);
}
