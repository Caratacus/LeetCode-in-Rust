// Tests for Problem 2133: Check if Every Row and Column Contains All Numbers
// Java reference: src/test/java/g2101_2200/s2133_check_if_every_row_and_column_contains_all_numbers/SolutionTest.java

use leetcode_in_rust::s2133::check_if_every_row_and_column_contains_all_numbers::Solution;

#[test]
fn test_check_valid() {
    assert_eq!(
        Solution::check_valid(vec![vec![1, 2, 3], vec![3, 1, 2], vec![2, 3, 1]]),
        true
    );
}

#[test]
fn test_check_valid2() {
    assert_eq!(
        Solution::check_valid(vec![vec![1, 1, 1], vec![1, 2, 3], vec![1, 2, 3]]),
        false
    );
}
