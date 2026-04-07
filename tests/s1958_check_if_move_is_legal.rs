// Tests for Problem 1958: Check If Move is Legal
// Java reference: src/test/java/g1901_2000/s1958_check_if_move_is_legal/SolutionTest.java

use leetcode_in_rust::s1958::check_if_move_is_legal::Solution;

#[test]
fn test_check_move() {
    let board = vec![
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
        vec!['W', 'B', 'B', '.', 'W', 'W', 'W', 'B'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'B', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', '.', '.', '.', '.'],
    ];
    assert_eq!(Solution::check_move(board, 4, 3, 'B'), true);
}

#[test]
fn test_check_move2() {
    let board = vec![
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', 'B', '.', '.', 'W', '.', '.', '.'],
        vec!['.', '.', 'W', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', 'W', 'B', '.', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', '.'],
        vec!['.', '.', '.', '.', 'B', 'W', '.', '.'],
        vec!['.', '.', '.', '.', '.', '.', 'W', '.'],
        vec!['.', '.', '.', '.', '.', '.', '.', 'B'],
    ];
    assert_eq!(Solution::check_move(board, 4, 4, 'W'), false);
}
