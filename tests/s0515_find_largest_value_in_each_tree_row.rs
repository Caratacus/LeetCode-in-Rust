// Tests for Problem 0515: Find Largest Value in Each Tree Row
// Java reference: src/test/java/g0501_0600/s0515_find_largest_value_in_each_tree_row/SolutionTest.java

use leetcode_in_rust::s0515::find_largest_value_in_each_tree_row::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_largest_values() {
    let root = tree_from_vec(vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)]);
    assert_eq!(Solution::largest_values(root), vec![1, 3, 9]);
}

#[test]
fn test_largest_values2() {
    let root = tree_from_vec(vec![Some(1), Some(2), Some(3)]);
    assert_eq!(Solution::largest_values(root), vec![1, 3]);
}
