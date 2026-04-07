// Tests for Problem 2577: Minimum Time to Visit a Cell in a Grid
// Java reference: src/test/java/g2501_2600/s2577_minimum_time_to_visit_a_cell_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2577::minimum_time_to_visit_a_cell_in_a_grid::Solution;

#[test]
fn test_minimum_time() {
    assert_eq!(
        Solution::minimum_time(vec![vec![0, 1, 3, 2], vec![5, 1, 2, 5], vec![4, 3, 8, 6]]),
        7
    );
}

#[test]
fn test_minimum_time2() {
    assert_eq!(
        Solution::minimum_time(vec![vec![0, 2, 4], vec![3, 2, 1], vec![1, 0, 4]]),
        -1
    );
}
