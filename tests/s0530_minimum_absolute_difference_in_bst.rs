// Tests for Problem 0530: Minimum Absolute Difference in BST
// Java reference: src/test/java/g0501_0600/s0530_minimum_absolute_difference_in_bst/SolutionTest.java

use leetcode_in_rust::s0530::minimum_absolute_difference_in_bst::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_get_minimum_difference() {
    let root = tree_from_vec(vec![Some(4), Some(2), Some(6), Some(1), Some(3)]);
    assert_eq!(Solution::get_minimum_difference(root), 1);
}

#[test]
fn test_get_minimum_difference2() {
    let root = tree_from_vec(vec![Some(1), Some(0), Some(48), None, None, Some(12), Some(49)]);
    assert_eq!(Solution::get_minimum_difference(root), 1);
}
