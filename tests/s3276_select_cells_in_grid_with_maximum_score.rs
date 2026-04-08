// Tests for Problem 3276: Select Cells in Grid With Maximum Score
// Java reference: src/test/java/g3201_3300/s3276_select_cells_in_grid_with_maximum_score/SolutionTest.java

use leetcode_in_rust::s3276::select_cells_in_grid_with_maximum_score::Solution;

#[test]
fn test_max_score() {
    assert_eq!(
        Solution::max_score(vec![vec![1, 2, 3], vec![4, 3, 2], vec![1, 1, 1]]),
        8
    );
}

#[test]
fn test_max_score2() {
    assert_eq!(
        Solution::max_score(vec![vec![8, 7, 6], vec![8, 3, 2]]),
        15
    );
}
