// Tests for Problem 2617: Minimum Number of Visited Cells in a Grid
// Java reference: src/test/java/g2601_2700/s2617_minimum_number_of_visited_cells_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2617::minimum_number_of_visited_cells_in_a_grid::Solution;

#[test]
fn test_minimum_visited_cells() {
    assert_eq!(
        Solution::minimum_visited_cells(vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 3, 1],
            vec![2, 1, 0, 0],
            vec![2, 4, 0, 0]
        ]),
        4
    );
}

#[test]
fn test_minimum_visited_cells2() {
    assert_eq!(
        Solution::minimum_visited_cells(vec![
            vec![3, 4, 2, 1],
            vec![4, 2, 1, 1],
            vec![2, 1, 1, 0],
            vec![3, 4, 1, 0]
        ]),
        3
    );
}

#[test]
fn test_minimum_visited_cells3() {
    assert_eq!(
        Solution::minimum_visited_cells(vec![vec![2, 1, 0], vec![1, 0, 0]]),
        -1
    );
}
