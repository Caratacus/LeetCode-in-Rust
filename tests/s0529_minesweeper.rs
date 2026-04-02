// Tests for Problem 0529: Minesweeper
// Java reference: src/test/java/g0501_0600/s0529_minesweeper/SolutionTest.java

use leetcode_in_rust::s0529::minesweeper::Solution;

#[test]
fn test_update_board() {
    let board = vec![
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'M', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
        vec!['E', 'E', 'E', 'E', 'E'],
    ];
    let expected = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'M', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    assert_eq!(Solution::update_board(board, vec![3, 0]), expected);
}

#[test]
fn test_update_board2() {
    let board = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'M', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    let expected = vec![
        vec!['B', '1', 'E', '1', 'B'],
        vec!['B', '1', 'X', '1', 'B'],
        vec!['B', '1', '1', '1', 'B'],
        vec!['B', 'B', 'B', 'B', 'B'],
    ];
    assert_eq!(Solution::update_board(board, vec![1, 2]), expected);
}
