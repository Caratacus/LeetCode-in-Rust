// Tests for Problem 1072: Flip Columns For Maximum Number of Equal Rows
// Java reference: src/test/java/g1001_1100/s1072_flip_columns_for_maximum_number_of_equal_rows/SolutionTest.java

use leetcode_in_rust::s1072::flip_columns_for_maximum_number_of_equal_rows::Solution;

#[test]
fn test_max_equal_rows_after_flips() {
    assert_eq!(Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 1]]), 1);
}

#[test]
fn test_max_equal_rows_after_flips2() {
    assert_eq!(Solution::max_equal_rows_after_flips(vec![vec![0, 1], vec![1, 0]]), 2);
}

#[test]
fn test_max_equal_rows_after_flips3() {
    assert_eq!(
        Solution::max_equal_rows_after_flips(vec![vec![0, 0, 0], vec![0, 0, 1], vec![1, 1, 0]]),
        2
    );
}
