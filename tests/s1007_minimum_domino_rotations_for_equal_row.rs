// Tests for Problem 1007: Minimum Domino Rotations For Equal Row
// Java reference: src/test/java/g1001_1100/s1007_minimum_domino_rotations_for_equal_row/SolutionTest.java

use leetcode_in_rust::s1007::minimum_domino_rotations_for_equal_row::Solution;

#[test]
fn test_min_domino_rotations() {
    assert_eq!(
        Solution::min_domino_rotations(vec![2, 1, 2, 4, 2, 2], vec![5, 2, 6, 2, 3, 2]),
        2
    );
}

#[test]
fn test_min_domino_rotations2() {
    assert_eq!(
        Solution::min_domino_rotations(vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4]),
        -1
    );
}
