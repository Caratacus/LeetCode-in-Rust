// Tests for Problem 0513: Find Bottom Left Tree Value
// Java reference: src/test/java/g0501_0600/s0513_find_bottom_left_tree_value/SolutionTest.java

use leetcode_in_rust::s0513::find_bottom_left_tree_value::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_find_bottom_left_value() {
    let root = tree_from_vec(vec![Some(2), Some(1), Some(3)]);
    assert_eq!(Solution::find_bottom_left_value(root), 1);
}

#[test]
fn test_find_bottom_left_value2() {
    let root = tree_from_vec(vec![
        Some(1), Some(2), Some(3), Some(4), None, Some(5), Some(6), None, None, Some(7)
    ]);
    assert_eq!(Solution::find_bottom_left_value(root), 7);
}
