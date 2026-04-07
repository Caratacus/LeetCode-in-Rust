// Tests for Problem 2397: Maximum Rows Covered by Columns
// Java reference: src/test/java/g2301_2400/s2397_maximum_rows_covered_by_columns/SolutionTest.java

use leetcode_in_rust::s2397::maximum_rows_covered_by_columns::Solution;

#[test]
fn test_maximum_rows() {
    assert_eq!(
        Solution::maximum_rows(
            vec![vec![0, 0, 0], vec![1, 0, 1], vec![0, 1, 1], vec![0, 0, 1]],
            2
        ),
        3
    );
}

#[test]
fn test_maximum_rows2() {
    assert_eq!(Solution::maximum_rows(vec![vec![1], vec![0]], 1), 2);
}
