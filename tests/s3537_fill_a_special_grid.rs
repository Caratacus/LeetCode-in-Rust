// Tests for Problem 3537: Fill a Special Grid
// Java reference: src/test/java/g3501_3600/s3537_fill_a_special_grid/SolutionTest.java

use leetcode_in_rust::s3537::fill_a_special_grid::Solution;

#[test]
fn test_special_grid() { assert_eq!(Solution::special_grid(0), vec![vec![0]]); }

#[test]
fn test_special_grid2() { assert_eq!(Solution::special_grid(1), vec![vec![3, 0], vec![2, 1]]); }

#[test]
fn test_special_grid3() {
    assert_eq!(Solution::special_grid(2), vec![vec![15, 12, 3, 0], vec![14, 13, 2, 1], vec![11, 8, 7, 4], vec![10, 9, 6, 5]]);
}
