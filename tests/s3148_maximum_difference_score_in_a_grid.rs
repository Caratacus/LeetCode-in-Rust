// Tests for Problem 3148: Maximum Difference Score in a Grid
// Java reference: src/test/java/g3101_3200/s3148_maximum_difference_score_in_a_grid/SolutionTest.java

use leetcode_in_rust::s3148::maximum_difference_score_in_a_grid::Solution;
#[test]
fn test_max_score() {
    assert_eq!(Solution::max_score(vec![vec![4, 3, 2], vec![0, 2]]), 9);
}
