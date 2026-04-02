// Tests for Problem 0144: Binary Tree Preorder Traversal
// Java reference: src/test/java/g0121_0200/s0144_binary_tree_preorder_traversal/SolutionTest.java

use leetcode_in_rust::s0144::binary_tree_preorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_preorder_traversal() {
    let root = tree_from_vec(vec![Some(1), None, Some(2), Some(3)]);
    assert_eq!(Solution::preorder_traversal(root), vec![1, 2, 3]);
}

#[test]
fn test_preorder_traversal2() {
    let root = tree_from_vec(vec![]);
    assert_eq!(Solution::preorder_traversal(root), vec![] as Vec<i32>);
}

#[test]
fn test_preorder_traversal3() {
    let root = tree_from_vec(vec![Some(1)]);
    assert_eq!(Solution::preorder_traversal(root), vec![1]);
}
