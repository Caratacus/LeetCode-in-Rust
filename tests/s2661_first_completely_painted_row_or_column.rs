// Tests for Problem 2661: First Completely Painted Row or Column
// Java reference: src/test/java/g2601_2700/s2661_first_completely_painted_row_or_column/SolutionTest.java

use leetcode_in_rust::s2661::first_completely_painted_row_or_column::Solution;

#[test]
fn test_first_complete_index() {
    assert_eq!(
        Solution::first_complete_index(vec![1, 3, 4, 2], vec![vec![1, 4], vec![2, 3]]),
        2
    );
}

#[test]
fn test_first_complete_index2() {
    assert_eq!(
        Solution::first_complete_index(
            vec![2, 8, 7, 4, 1, 3, 5, 6, 9],
            vec![vec![3, 2, 5], vec![1, 4, 6], vec![8, 7, 9]]
        ),
        3
    );
}
