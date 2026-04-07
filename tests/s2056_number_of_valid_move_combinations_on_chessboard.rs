// Tests for Problem 2056: Number of Valid Move Combinations on Chessboard
// Java reference: src/test/java/g2001_2100/s2056_number_of_valid_move_combinations_on_chessboard/SolutionTest.java

use leetcode_in_rust::s2056::number_of_valid_move_combinations_on_chessboard::Solution;

#[test]
fn test_count_combinations() {
    assert_eq!(
        Solution::count_combinations(vec!["rook".to_string()], vec![vec![1, 1]]),
        15
    );
}

#[test]
fn test_count_combinations2() {
    assert_eq!(
        Solution::count_combinations(vec!["queen".to_string()], vec![vec![1, 1]]),
        22
    );
}

#[test]
fn test_count_combinations3() {
    assert_eq!(
        Solution::count_combinations(vec!["bishop".to_string()], vec![vec![4, 3]]),
        12
    );
}
