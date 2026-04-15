// Tests for Problem 3417: Zigzag Grid Traversal with Skip
// Java reference: src/test/java/g3401_3500/s3417_zigzag_grid_traversal_with_skip/SolutionTest.java

use leetcode_in_rust::s3417::zigzag_grid_traversal_with_skip::Solution;

#[test]
fn test_zigzag_traversal() {
    assert_eq!(Solution::zigzag_traversal(vec![vec![1, 2], vec![3, 4]]), vec![1, 4]);
}

#[test]
fn test_zigzag_traversal2() {
    assert_eq!(Solution::zigzag_traversal(vec![vec![2, 1], vec![2, 1], vec![2, 1]]), vec![2, 1, 2]);
}

#[test]
fn test_zigzag_traversal3() {
    assert_eq!(Solution::zigzag_traversal(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]), vec![1, 3, 5, 7, 9]);
}
