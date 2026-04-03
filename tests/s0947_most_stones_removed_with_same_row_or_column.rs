// Tests for Problem 0947: Most Stones Removed with Same Row or Column
// Java reference: src/test/java/g0901_1000/s0947_most_stones_removed_with_same_row_or_column/SolutionTest.java

use leetcode_in_rust::s0947::most_stones_removed_with_same_row_or_column::Solution;

#[test]
fn test_remove_stones() {
    let result = Solution::remove_stones(vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]]);
    assert_eq!(result, 5);
}

#[test]
fn test_remove_stones2() {
    let result = Solution::remove_stones(vec![vec![0, 0], vec![0, 2], vec![1, 1], vec![2, 0], vec![2, 2]]);
    assert_eq!(result, 3);
}

#[test]
fn test_remove_stones3() {
    let result = Solution::remove_stones(vec![vec![0, 0]]);
    assert_eq!(result, 0);
}
