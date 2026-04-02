// Tests for Problem 0079: Word Search
// Java reference: src/test/java/g0001_0100/s0079_word_search/SolutionTest.java

use leetcode_in_rust::s0079::word_search::Solution;

#[test]
fn test_exist() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(Solution::exist(board, "ABCCED".to_string()), true);
}

#[test]
fn test_exist2() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(Solution::exist(board, "SEE".to_string()), true);
}

#[test]
fn test_exist3() {
    let board = vec![
        vec!['A', 'B', 'C', 'E'],
        vec!['S', 'F', 'C', 'S'],
        vec!['A', 'D', 'E', 'E'],
    ];
    assert_eq!(Solution::exist(board, "ABCB".to_string()), false);
}
