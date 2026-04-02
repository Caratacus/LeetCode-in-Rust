// Tests for Problem 0094: Binary Tree Inorder Traversal
// Java reference: src/test/java/g0001_0100/s0094_binary_tree_inorder_traversal/SolutionTest.java

use leetcode_in_rust::s0094::binary_tree_inorder_traversal::Solution;
use leetcode_in_rust::utils::tree_utils::tree_from_vec;

#[test]
fn test_inorder_traversal() {
    let root = tree_from_vec(vec![Some(1), None, Some(2), Some(3)]);
    let result = Solution::inorder_traversal(root);
    assert_eq!(result, vec![1, 3, 2]);
}

#[test]
fn test_inorder_traversal2() {
    let root = tree_from_vec(vec![]);
    let result = Solution::inorder_traversal(root);
    assert_eq!(result, vec![] as Vec<i32>);
}

#[test]
fn test_inorder_traversal3() {
    let root = tree_from_vec(vec![Some(1)]);
    let result = Solution::inorder_traversal(root);
    assert_eq!(result, vec![1]);
}
