// Tests for Problem 2684: Maximum Number of Moves in a Grid
// Java reference: src/test/java/g2601_2700/s2684_maximum_number_of_moves_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2684::maximum_number_of_moves_in_a_grid::Solution;

#[test]
fn test_max_moves() {
    assert_eq!(
        Solution::max_moves(vec![
            vec![2, 4, 3, 5],
            vec![5, 4, 9, 3],
            vec![3, 4, 2, 11],
            vec![10, 9, 13, 15]
        ]),
        3
    );
}

#[test]
fn test_max_moves2() {
    assert_eq!(
        Solution::max_moves(vec![vec![3, 2, 4], vec![2, 1, 9], vec![1, 1, 7]]),
        0
    );
}
