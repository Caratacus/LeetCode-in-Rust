// Tests for Problem 0107: Binary Tree Level Order Traversal II
// Java reference: src/test/java/g0101_0200/s0107_binary_tree_level_order_traversal_ii/SolutionTest.java

use leetcode_in_rust::s0107::binary_tree_level_order_traversal_ii::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_level_order_bottom() {
    let root = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::level_order_bottom(root), vec![vec![15, 7], vec![9, 20], vec![3]]);
}

#[test]
fn test_level_order_bottom2() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::level_order_bottom(root), vec![vec![1]]);
}

#[test]
fn test_level_order_bottom3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::level_order_bottom(root), vec![] as Vec<Vec<i32>>);
}
