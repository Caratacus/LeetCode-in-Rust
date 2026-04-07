// Tests for Problem 2352: Equal Row and Column Pairs
// Java reference: src/test/java/g2301_2400/s2352_equal_row_and_column_pairs/SolutionTest.java

use leetcode_in_rust::s2352::equal_row_and_column_pairs::Solution;

#[test]
fn test_equal_pairs() {
    assert_eq!(
        Solution::equal_pairs(vec![vec![3, 2, 1], vec![1, 7, 6], vec![2, 7, 7]]),
        1
    );
}

#[test]
fn test_equal_pairs2() {
    assert_eq!(
        Solution::equal_pairs(vec![
            vec![3, 1, 2, 2],
            vec![1, 4, 4, 5],
            vec![2, 4, 2, 2],
            vec![2, 4, 2, 2]
        ]),
        3
    );
}
