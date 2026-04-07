// Tests for Problem 2257: Count Unguarded Cells in the Grid
// Java reference: src/test/java/g2201_2300/s2257_count_unguarded_cells_in_the_grid/SolutionTest.java

use leetcode_in_rust::s2257::count_unguarded_cells_in_the_grid::Solution;

#[test]
fn test_count_unguarded() {
    assert_eq!(
        Solution::count_unguarded(
            4,
            6,
            vec![vec![0, 0], vec![1, 1], vec![2, 3]],
            vec![vec![0, 1], vec![2, 2], vec![1, 4]]
        ),
        7
    );
}

#[test]
fn test_count_unguarded2() {
    assert_eq!(
        Solution::count_unguarded(
            3,
            3,
            vec![vec![1, 1]],
            vec![vec![0, 1], vec![1, 0], vec![2, 1], vec![1, 2]]
        ),
        4
    );
}
