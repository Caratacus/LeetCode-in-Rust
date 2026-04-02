// Tests for Problem 0419: Battleships in a Board
// Java reference: src/test/java/g0401_0500/s0419_battleships_in_a_board/SolutionTest.java

use leetcode_in_rust::s0419::battleships_in_a_board::Solution;

#[test]
fn test_count_battleships() {
    let board = vec![
        vec!['X', '.', '.', 'X'],
        vec!['.', '.', '.', 'X'],
        vec!['.', '.', '.', 'X'],
    ];
    assert_eq!(Solution::count_battleships(board), 2);
}

#[test]
fn test_count_battleships2() {
    let board = vec![vec!['.']];
    assert_eq!(Solution::count_battleships(board), 0);
}
