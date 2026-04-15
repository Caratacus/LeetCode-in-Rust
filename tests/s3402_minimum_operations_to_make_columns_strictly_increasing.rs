// Tests for Problem 3402: Minimum Operations to Make Columns Strictly Increasing
// Java reference: src/test/java/g3401_3500/s3402_minimum_operations_to_make_columns_strictly_increasing/SolutionTest.java

use leetcode_in_rust::s3402::minimum_operations_to_make_columns_strictly_increasing::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(
        Solution::minimum_operations(vec![vec![3, 2], vec![1, 3], vec![3, 4], vec![0, 1]]),
        15
    );
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(
        Solution::minimum_operations(vec![vec![3, 2, 1], vec![2, 1, 0], vec![1, 2, 3]]),
        12
    );
}
