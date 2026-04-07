// Tests for Problem 2639: Find the Width of Columns of a Grid
// Java reference: src/test/java/g2601_2700/s2639_find_the_width_of_columns_of_a_grid/SolutionTest.java

use leetcode_in_rust::s2639::find_the_width_of_columns_of_a_grid::Solution;

#[test]
fn test_find_column_width() {
    assert_eq!(
        Solution::find_column_width(vec![vec![1], vec![22], vec![333]]),
        vec![3]
    );
}

#[test]
fn test_find_column_width2() {
    assert_eq!(
        Solution::find_column_width(vec![vec![-15, 1, 3], vec![15, 7, 12], vec![5, 6, -2]]),
        vec![3, 1, 2]
    );
}
