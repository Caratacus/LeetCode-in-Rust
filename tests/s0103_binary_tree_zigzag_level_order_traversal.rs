// Tests for Problem 0103: Binary Tree Zigzag Level Order Traversal
// Java reference: src/test/java/g0101_0200/s0103_binary_tree_zigzag_level_order_traversal/SolutionTest.java

use leetcode_in_rust::s0103::binary_tree_zigzag_level_order_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_zigzag_level_order() {
    let root = tree_from_vec(vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)]);
    assert_eq!(Solution::zigzag_level_order(root), vec![vec![3], vec![20, 9], vec![15, 7]]);
}

#[test]
fn test_zigzag_level_order2() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::zigzag_level_order(root), vec![vec![1]]);
}

#[test]
fn test_zigzag_level_order3() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::zigzag_level_order(root), vec![] as Vec<Vec<i32>>);
}
