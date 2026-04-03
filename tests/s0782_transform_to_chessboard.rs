// Tests for Problem 0782: Transform to Chessboard
// Java reference: src/test/java/g0701_0800/s0782_transform_to_chessboard/SolutionTest.java

use leetcode_in_rust::s0782::transform_to_chessboard::Solution;

#[test]
fn test_moves_to_chessboard() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1]
        ]),
        2
    );
}

#[test]
fn test_moves_to_chessboard2() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![0, 1], vec![1, 0]]),
        0
    );
}

#[test]
fn test_moves_to_chessboard3() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![1, 0], vec![1, 0]]),
        -1
    );
}

#[test]
fn test_moves_to_chessboard4() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![0, 0], vec![0, 1]]),
        -1
    );
}

#[test]
fn test_moves_to_chessboard5() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![1, 1], vec![1, 1]]),
        -1
    );
}

#[test]
fn test_moves_to_chessboard6() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![1, 0], vec![0, 1]]),
        0
    );
}

#[test]
fn test_moves_to_chessboard7() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![vec![1, 0, 1], vec![0, 1, 0], vec![1, 0, 1]]),
        0
    );
}

#[test]
fn test_moves_to_chessboard8() {
    assert_eq!(
        Solution::moves_to_chessboard(vec![
            vec![1, 0, 0, 1],
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1]
        ]),
        0
    );
}

#[test]
fn test_moves_to_chessboard9() {
    assert_eq!(Solution::moves_to_chessboard(vec![vec![0]]), 0);
}
