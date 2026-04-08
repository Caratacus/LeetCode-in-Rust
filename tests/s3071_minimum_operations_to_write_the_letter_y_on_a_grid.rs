// Tests for Problem 3071: Minimum Operations to Write the Letter Y on a Grid
// Java reference: src/test/java/g3001_3100/s3071_minimum_operations_to_write_the_letter_y_on_a_grid/SolutionTest.java

use leetcode_in_rust::s3071::minimum_operations_to_write_the_letter_y_on_a_grid::Solution;

#[test]
fn test_minimum_operations_to_write_y() {
    assert_eq!(
        Solution::minimum_operations_to_write_y(vec![vec![1, 2, 2], vec![1, 1, 0], vec![0, 1, 0]]),
        3
    );
}

#[test]
fn test_minimum_operations_to_write_y2() {
    assert_eq!(
        Solution::minimum_operations_to_write_y(vec![
            vec![0, 1, 0, 1, 0],
            vec![2, 1, 0, 1, 2],
            vec![2, 2, 2, 0, 1],
            vec![2, 2, 2, 2, 2],
            vec![2, 1, 2, 2, 2]
        ]),
        12
    );
}
